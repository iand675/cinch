# Crypto.P256

Constant-time NIST P-256 keys, SEC 1 point encodings, ECDH and scalar/point arithmetic over generated Montgomery kernels.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.P256</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.P256/source/">View source</a><a href="/packages/search/?q=%2BCrypto.P256">Search module</a><a href="/reference-sources/Crypto.P256.cinch" download="Crypto.P256.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

The NIST P-256 curve (secp256r1): constant-time field, scalar and point arithmetic, keys and ECDH.
Field elements and scalars are fixed 256-bit values in Montgomery form, held as
two U128 halves and operated on by generated straight-line limb kernels, so
no arithmetic branches on secret data. Points use projective coordinates and
the complete addition and doubling formulas for a = -3 (Renes, Costello and
Batina), which have no exceptional cases; scalar multiplication is a fixed
4-bit window whose table entry is chosen by scanning every entry. Public keys
are SEC 1 points (uncompressed, or compressed on input), checked to lie on
the curve; private keys are scalars in [1, n - 1]. ECDH yields the shared
point's x-coordinate. Crypto.Ecdsa signs and verifies over this module.

</details>

<details class="doc-synopsis"><summary>Synopsis (28 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-P256Error">P256Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">P256Error</strong></code></li><li><a class="syn-name" href="#t-P256Point">P256Point</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">P256Point</strong></code></li><li><a class="syn-name" href="#t-P256PrivateKey">P256PrivateKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">P256PrivateKey</strong></code></li><li><a class="syn-name" href="#t-P256PublicKey">P256PublicKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">P256PublicKey</strong></code></li><li><a class="syn-name" href="#t-P256Scalar">P256Scalar</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">P256Scalar</strong></code></li><li><a class="syn-name" href="#v-p256-base-mult">p256-base-mult</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-base-mult</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a></code></li><li><a class="syn-name" href="#v-p256-ecdh">p256-ecdh</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-ecdh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-p256-error-message">p256-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-p256-point-add">p256-point-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-point-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a></code></li><li><a class="syn-name" href="#v-p256-point-is-identity">p256-point-is-identity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-point-is-identity</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-p256-point-mult">p256-point-mult</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-point-mult</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a></code></li><li><a class="syn-name" href="#v-p256-point-x-scalar">p256-point-x-scalar</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-point-x-scalar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code></li><li><a class="syn-name" href="#v-p256-private-key">p256-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a></code></li><li><a class="syn-name" href="#v-p256-private-key-bytes">p256-private-key-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-private-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-p256-private-scalar">p256-private-scalar</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-private-scalar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code></li><li><a class="syn-name" href="#v-p256-public-key">p256-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a></code></li><li><a class="syn-name" href="#v-p256-public-key-bytes">p256-public-key-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-public-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-p256-public-key-compressed">p256-public-key-compressed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-public-key-compressed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-p256-public-key-coordinates">p256-public-key-coordinates</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-public-key-coordinates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-p256-public-key-of">p256-public-key-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-public-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a></code></li><li><a class="syn-name" href="#v-p256-public-point">p256-public-point</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-public-point</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a></code></li><li><a class="syn-name" href="#v-p256-scalar">p256-scalar</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-scalar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code></li><li><a class="syn-name" href="#v-p256-scalar-add">p256-scalar-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-scalar-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code></li><li><a class="syn-name" href="#v-p256-scalar-bytes">p256-scalar-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-scalar-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-p256-scalar-inverse">p256-scalar-inverse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-scalar-inverse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code></li><li><a class="syn-name" href="#v-p256-scalar-is-zero">p256-scalar-is-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-scalar-is-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-p256-scalar-mul">p256-scalar-mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-scalar-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code></li><li><a class="syn-name" href="#v-p256-scalar-reduce">p256-scalar-reduce</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">p256-scalar-reduce</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-P256Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">P256Error</strong></code><span class="doc-sig-links"><a href="#t-P256Error" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

A rejected key or scalar.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-P256InvalidPrivateKey"><code><strong class="tok-constructor">P256InvalidPrivateKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a></code></li><li id="k-P256InvalidPublicKey"><code><strong class="tok-constructor">P256InvalidPublicKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a></code></li><li id="k-P256InvalidScalar"><code><strong class="tok-constructor">P256InvalidScalar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-P256Point">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">P256Point</strong></code><span class="doc-sig-links"><a href="#t-P256Point" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L174" title="View source at line 174">Source</a></span></div>
<div class="doc-body">

The point.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-P256Point"><code><strong class="tok-constructor">P256Point</strong> :: <span class="tok-constructor">Point</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-P256PrivateKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">P256PrivateKey</strong></code><span class="doc-sig-links"><a href="#t-P256PrivateKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Scalar, then bytes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-P256PrivateKey"><code><strong class="tok-constructor">P256PrivateKey</strong> :: <span class="tok-constructor">Sc</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-P256PublicKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">P256PublicKey</strong></code><span class="doc-sig-links"><a href="#t-P256PublicKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

x, then y.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-P256PublicKey"><code><strong class="tok-constructor">P256PublicKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Field25519/#t-Fe" title="Crypto.P256.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Field25519/#t-Fe" title="Crypto.P256.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-P256Scalar">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">P256Scalar</strong></code><span class="doc-sig-links"><a href="#t-P256Scalar" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L138" title="View source at line 138">Source</a></span></div>
<div class="doc-body">

The Montgomery-form value.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-P256Scalar"><code><strong class="tok-constructor">P256Scalar</strong> :: <span class="tok-constructor">Sc</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-p256-base-mult">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-base-mult</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a></code><span class="doc-sig-links"><a href="#v-p256-base-mult" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

s * G.

</div>
</article>

<article class="doc-entry" id="v-p256-ecdh">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-ecdh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-p256-ecdh" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L106" title="View source at line 106">Source</a></span></div>
<div class="doc-body">

ECDH: the x-coordinate (32 bytes) of the private scalar times the peer's point.

</div>
</article>

<article class="doc-entry" id="v-p256-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-p256-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

Render an error.

</div>
</article>

<article class="doc-entry" id="v-p256-point-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-point-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a></code><span class="doc-sig-links"><a href="#v-p256-point-add" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L189" title="View source at line 189">Source</a></span></div>
<div class="doc-body">

P + Q.

</div>
</article>

<article class="doc-entry" id="v-p256-point-is-identity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-point-is-identity</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-p256-point-is-identity" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

Whether a point is the identity.

</div>
</article>

<article class="doc-entry" id="v-p256-point-mult">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-point-mult</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a></code><span class="doc-sig-links"><a href="#v-p256-point-mult" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L185" title="View source at line 185">Source</a></span></div>
<div class="doc-body">

s * P.

</div>
</article>

<article class="doc-entry" id="v-p256-point-x-scalar">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-point-x-scalar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code><span class="doc-sig-links"><a href="#v-p256-point-x-scalar" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L197" title="View source at line 197">Source</a></span></div>
<div class="doc-body">

The affine x-coordinate reduced modulo n (None for the identity).

</div>
</article>

<article class="doc-entry" id="v-p256-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a></code><span class="doc-sig-links"><a href="#v-p256-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

Parse a 32-byte big-endian private key in [1, n - 1].

</div>
</article>

<article class="doc-entry" id="v-p256-private-key-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-private-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-p256-private-key-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

The private key's 32 bytes.

</div>
</article>

<article class="doc-entry" id="v-p256-private-scalar">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-private-scalar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code><span class="doc-sig-links"><a href="#v-p256-private-scalar" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

The private key's scalar.

</div>
</article>

<article class="doc-entry" id="v-p256-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a></code><span class="doc-sig-links"><a href="#v-p256-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

Parse a SEC 1 public key: 0x04 || x || y (65 bytes) or 0x02/0x03 || x (33 bytes); the point must be on the curve.

</div>
</article>

<article class="doc-entry" id="v-p256-public-key-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-public-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-p256-public-key-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

The uncompressed encoding (65 bytes).

</div>
</article>

<article class="doc-entry" id="v-p256-public-key-compressed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-public-key-compressed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-p256-public-key-compressed" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

The compressed encoding (33 bytes).

</div>
</article>

<article class="doc-entry" id="v-p256-public-key-coordinates">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-public-key-coordinates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-p256-public-key-coordinates" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

The affine coordinates as 32-byte big-endian values.

</div>
</article>

<article class="doc-entry" id="v-p256-public-key-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-public-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a></code><span class="doc-sig-links"><a href="#v-p256-public-key-of" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L73" title="View source at line 73">Source</a></span></div>
<div class="doc-body">

The public key d * G.

</div>
</article>

<article class="doc-entry" id="v-p256-public-point">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-public-point</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Point" title="Crypto.P256.P256Point">P256Point</a></code><span class="doc-sig-links"><a href="#v-p256-public-point" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

A public key's point.

</div>
</article>

<article class="doc-entry" id="v-p256-scalar">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-scalar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Error" title="Crypto.P256.P256Error">P256Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code><span class="doc-sig-links"><a href="#v-p256-scalar" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

Parse a 32-byte big-endian scalar below n.

</div>
</article>

<article class="doc-entry" id="v-p256-scalar-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-scalar-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code><span class="doc-sig-links"><a href="#v-p256-scalar-add" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L164" title="View source at line 164">Source</a></span></div>
<div class="doc-body">

Sum modulo n.

</div>
</article>

<article class="doc-entry" id="v-p256-scalar-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-scalar-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-p256-scalar-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L152" title="View source at line 152">Source</a></span></div>
<div class="doc-body">

The scalar's 32-byte big-endian encoding.

</div>
</article>

<article class="doc-entry" id="v-p256-scalar-inverse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-scalar-inverse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code><span class="doc-sig-links"><a href="#v-p256-scalar-inverse" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L168" title="View source at line 168">Source</a></span></div>
<div class="doc-body">

Inverse modulo n (zero maps to zero).

</div>
</article>

<article class="doc-entry" id="v-p256-scalar-is-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-scalar-is-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-p256-scalar-is-zero" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L156" title="View source at line 156">Source</a></span></div>
<div class="doc-body">

Whether the scalar is zero.

</div>
</article>

<article class="doc-entry" id="v-p256-scalar-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-scalar-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code><span class="doc-sig-links"><a href="#v-p256-scalar-mul" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

Product modulo n.

</div>
</article>

<article class="doc-entry" id="v-p256-scalar-reduce">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">p256-scalar-reduce</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code><span class="doc-sig-links"><a href="#v-p256-scalar-reduce" title="Permalink">#</a><a href="/packages/stdlib/Crypto.P256/source/#L146" title="View source at line 146">Source</a></span></div>
<div class="doc-body">

Interpret up to the first 32 bytes as a big-endian integer and reduce it modulo n (shorter inputs are left-padded).

</div>
</article>
