# Crypto.Rsa

RSA PKCS#1 v1.5 and PSS signing/verification, OAEP and PKCS#1 v1.5 encryption/decryption, PKCS#1 DER key codecs and seeded key generation.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.Rsa</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.Rsa/source/">View source</a><a href="/packages/search/?q=%2BCrypto.Rsa">Search module</a><a href="/reference-sources/Crypto.Rsa.cinch" download="Crypto.Rsa.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Asn1](Text.Asn1.md), [Text.Md5](Text.Md5.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md).

<details class="doc-notes"><summary>Module notes</summary>

RSA public-key cryptography (RFC 8017): PKCS#1 v1.5 and RSASSA-PSS signatures,
RSAES-OAEP and RSAES-PKCS1-v1_5 encryption, PKCS#1 DER key serialization, key
validation and deterministic seeded key generation.

Private-key modular exponentiation uses Chinese Remainder Theorem (CRT)
recombination with a public-exponent consistency check against faults. Padding
checks in decryption and signature verification use constant-time comparisons
from Crypto.ConstantTime; underlying arbitrary-precision arithmetic uses Data.Nat
and is not constant-time in operand values.

</details>

<details class="doc-synopsis"><summary>Synopsis (44 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-PssSaltLength">PssSaltLength</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PssSaltLength</strong></code></li><li><a class="syn-name" href="#t-RsaError">RsaError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RsaError</strong></code></li><li><a class="syn-name" href="#t-RsaHash">RsaHash</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RsaHash</strong></code></li><li><a class="syn-name" href="#t-RsaPrivateKey">RsaPrivateKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RsaPrivateKey</strong></code></li><li><a class="syn-name" href="#t-RsaPublicKey">RsaPublicKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RsaPublicKey</strong></code></li><li><a class="syn-name" href="#v-rsa-decrypt-oaep">rsa-decrypt-oaep</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-decrypt-oaep</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-decrypt-oaep-mgf">rsa-decrypt-oaep-mgf</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-decrypt-oaep-mgf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-decrypt-pkcs1v15">rsa-decrypt-pkcs1v15</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-decrypt-pkcs1v15</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-decrypt-pkcs1v15-session-key">rsa-decrypt-pkcs1v15-session-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-decrypt-pkcs1v15-session-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-encrypt-oaep-mgf">rsa-encrypt-oaep-mgf</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-encrypt-oaep-mgf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-encrypt-oaep-with-seed">rsa-encrypt-oaep-with-seed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-encrypt-oaep-with-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-encrypt-pkcs1v15-from-seed">rsa-encrypt-pkcs1v15-from-seed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-encrypt-pkcs1v15-from-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-encrypt-pkcs1v15-with-pad">rsa-encrypt-pkcs1v15-with-pad</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-encrypt-pkcs1v15-with-pad</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-error-message">rsa-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-rsa-generate-key-from-seed">rsa-generate-key-from-seed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-generate-key-from-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a></code></li><li><a class="syn-name" href="#v-rsa-hash-size">rsa-hash-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-hash-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-hash-sum">rsa-hash-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-hash-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-i2osp">rsa-i2osp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-i2osp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-key-bits">rsa-key-bits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-key-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-key-size">rsa-key-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-marshal-pkcs1-private-key">rsa-marshal-pkcs1-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-marshal-pkcs1-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-marshal-pkcs1-public-key">rsa-marshal-pkcs1-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-marshal-pkcs1-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-os2ip">rsa-os2ip</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-os2ip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-parse-pkcs1-private-key">rsa-parse-pkcs1-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-parse-pkcs1-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a></code></li><li><a class="syn-name" href="#v-rsa-parse-pkcs1-public-key">rsa-parse-pkcs1-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-parse-pkcs1-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a></code></li><li><a class="syn-name" href="#v-rsa-private-d">rsa-private-d</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-private-d</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-private-dp">rsa-private-dp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-private-dp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-private-dq">rsa-private-dq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-private-dq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-private-key">rsa-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a></code></li><li><a class="syn-name" href="#v-rsa-private-key-equal">rsa-private-key-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-private-key-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-rsa-private-p">rsa-private-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-private-p</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-private-q">rsa-private-q</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-private-q</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-private-qinv">rsa-private-qinv</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-private-qinv</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-public-e">rsa-public-e</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-public-e</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-public-key">rsa-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a></code></li><li><a class="syn-name" href="#v-rsa-public-key-equal">rsa-public-key-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-public-key-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-rsa-public-key-of">rsa-public-key-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-public-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a></code></li><li><a class="syn-name" href="#v-rsa-public-n">rsa-public-n</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-public-n</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rsa-sign-pkcs1v15">rsa-sign-pkcs1v15</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-sign-pkcs1v15</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-sign-pss">rsa-sign-pss</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-sign-pss</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-PssSaltLength" title="Crypto.Rsa.PssSaltLength">PssSaltLength</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-sign-pss-deterministic">rsa-sign-pss-deterministic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-sign-pss-deterministic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rsa-validate-private-key">rsa-validate-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-validate-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-rsa-verify-pkcs1v15">rsa-verify-pkcs1v15</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-verify-pkcs1v15</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-rsa-verify-pss">rsa-verify-pss</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rsa-verify-pss</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-PssSaltLength" title="Crypto.Rsa.PssSaltLength">PssSaltLength</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-PssSaltLength">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PssSaltLength</strong></code><span class="doc-sig-links"><a href="#t-PssSaltLength" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Salt length policy for RSASSA-PSS signing and verification.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PssSaltAuto"><code><strong class="tok-constructor">PssSaltAuto</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-PssSaltLength" title="Crypto.Rsa.PssSaltLength">PssSaltLength</a></code></li><li id="k-PssSaltEqualsHash"><code><strong class="tok-constructor">PssSaltEqualsHash</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-PssSaltLength" title="Crypto.Rsa.PssSaltLength">PssSaltLength</a></code></li><li id="k-PssSaltExact"><code><strong class="tok-constructor">PssSaltExact</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-PssSaltLength" title="Crypto.Rsa.PssSaltLength">PssSaltLength</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-RsaError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RsaError</strong></code><span class="doc-sig-links"><a href="#t-RsaError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

Errors produced by RSA key handling, encoding, signing, verification and encryption.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-RsaMessageTooLong"><code><strong class="tok-constructor">RsaMessageTooLong</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a></code></li><li id="k-RsaDecryptionError"><code><strong class="tok-constructor">RsaDecryptionError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a></code></li><li id="k-RsaVerificationError"><code><strong class="tok-constructor">RsaVerificationError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a></code></li><li id="k-RsaInvalidPublicKey"><code><strong class="tok-constructor">RsaInvalidPublicKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a></code></li><li id="k-RsaInvalidPrivateKey"><code><strong class="tok-constructor">RsaInvalidPrivateKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a></code></li><li id="k-RsaInvalidDigestLength"><code><strong class="tok-constructor">RsaInvalidDigestLength</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a></code></li><li id="k-RsaUnsupportedHash"><code><strong class="tok-constructor">RsaUnsupportedHash</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a></code></li><li id="k-RsaInvalidParameter"><code><strong class="tok-constructor">RsaInvalidParameter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a></code></li><li id="k-RsaInvalidEncoding"><code><strong class="tok-constructor">RsaInvalidEncoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-RsaHash">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RsaHash</strong></code><span class="doc-sig-links"><a href="#t-RsaHash" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

Hash algorithm used by RSA signature and OAEP padding schemes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-RsaNoHash"><code><strong class="tok-constructor">RsaNoHash</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a></code></li><li id="k-RsaMd5"><code><strong class="tok-constructor">RsaMd5</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a></code></li><li id="k-RsaSha1"><code><strong class="tok-constructor">RsaSha1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a></code></li><li id="k-RsaSha224"><code><strong class="tok-constructor">RsaSha224</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a></code></li><li id="k-RsaSha256"><code><strong class="tok-constructor">RsaSha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a></code></li><li id="k-RsaSha384"><code><strong class="tok-constructor">RsaSha384</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a></code></li><li id="k-RsaSha512"><code><strong class="tok-constructor">RsaSha512</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a></code></li><li id="k-RsaSha512t224"><code><strong class="tok-constructor">RsaSha512t224</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a></code></li><li id="k-RsaSha512t256"><code><strong class="tok-constructor">RsaSha512t256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-RsaPrivateKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RsaPrivateKey</strong></code><span class="doc-sig-links"><a href="#t-RsaPrivateKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

A validated two-prime RSA private key with precomputed CRT values.

</div>
</article>

<article class="doc-entry" id="t-RsaPublicKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RsaPublicKey</strong></code><span class="doc-sig-links"><a href="#t-RsaPublicKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

A validated RSA public key holding modulus n and public exponent e.

</div>
</article>

# Values

<article class="doc-entry" id="v-rsa-decrypt-oaep">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-decrypt-oaep</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-decrypt-oaep" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L954" title="View source at line 954">Source</a></span></div>
<div class="doc-body">

Decrypt an RSAES-OAEP ciphertext using the same hash for the label and MGF1, with constant-time padding checks.

</div>
</article>

<article class="doc-entry" id="v-rsa-decrypt-oaep-mgf">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-decrypt-oaep-mgf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-decrypt-oaep-mgf" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L958" title="View source at line 958">Source</a></span></div>
<div class="doc-body">

Decrypt an RSAES-OAEP ciphertext with separate label and MGF1 hashes and constant-time padding validation.

</div>
</article>

<article class="doc-entry" id="v-rsa-decrypt-pkcs1v15">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-decrypt-pkcs1v15</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-decrypt-pkcs1v15" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L1058" title="View source at line 1058">Source</a></span></div>
<div class="doc-body">

Decrypt an RSAES-PKCS1-v1_5 ciphertext with constant-time padding validation.

</div>
</article>

<article class="doc-entry" id="v-rsa-decrypt-pkcs1v15-session-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-decrypt-pkcs1v15-session-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-decrypt-pkcs1v15-session-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L1068" title="View source at line 1068">Source</a></span></div>
<div class="doc-body">

Decrypt an RSAES-PKCS1-v1_5 session key of fixed length, returning fallback if padding or length is invalid.

</div>
</article>

<article class="doc-entry" id="v-rsa-encrypt-oaep-mgf">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-encrypt-oaep-mgf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-encrypt-oaep-mgf" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L924" title="View source at line 924">Source</a></span></div>
<div class="doc-body">

Encrypt a message with RSAES-OAEP using separate label and MGF1 hashes, and explicit seed bytes.

</div>
</article>

<article class="doc-entry" id="v-rsa-encrypt-oaep-with-seed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-encrypt-oaep-with-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-encrypt-oaep-with-seed" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L920" title="View source at line 920">Source</a></span></div>
<div class="doc-body">

Encrypt a message with RSAES-OAEP using the same hash for the label and MGF1, and explicit seed bytes.

</div>
</article>

<article class="doc-entry" id="v-rsa-encrypt-pkcs1v15-from-seed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-encrypt-pkcs1v15-from-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-encrypt-pkcs1v15-from-seed" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L1039" title="View source at line 1039">Source</a></span></div>
<div class="doc-body">

Encrypt a message with RSAES-PKCS1-v1_5, deriving non-zero padding bytes deterministically from seed.

</div>
</article>

<article class="doc-entry" id="v-rsa-encrypt-pkcs1v15-with-pad">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-encrypt-pkcs1v15-with-pad</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-encrypt-pkcs1v15-with-pad" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L1020" title="View source at line 1020">Source</a></span></div>
<div class="doc-body">

Encrypt a message with RSAES-PKCS1-v1_5 using explicit non-zero padding bytes of length k - len(msg) - 3.

</div>
</article>

<article class="doc-entry" id="v-rsa-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-rsa-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

Render an RSA error as a human-readable message.

</div>
</article>

<article class="doc-entry" id="v-rsa-generate-key-from-seed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-generate-key-from-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a></code><span class="doc-sig-links"><a href="#v-rsa-generate-key-from-seed" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L1186" title="View source at line 1186">Source</a></span></div>
<div class="doc-body">

Generate a validated two-prime RSA private key of the requested bit size (64..8192) deterministically from seed.

</div>
</article>

<article class="doc-entry" id="v-rsa-hash-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-hash-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-hash-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L418" title="View source at line 418">Source</a></span></div>
<div class="doc-body">

Output digest length in bytes for an RSA hash algorithm (0 for RsaNoHash).

</div>
</article>

<article class="doc-entry" id="v-rsa-hash-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-hash-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-hash-sum" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L431" title="View source at line 431">Source</a></span></div>
<div class="doc-body">

Compute a hash digest of a message using the specified RSA hash algorithm.

</div>
</article>

<article class="doc-entry" id="v-rsa-i2osp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-i2osp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-i2osp" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L250" title="View source at line 250">Source</a></span></div>
<div class="doc-body">

Convert a Nat to a fixed-length big-endian byte string, zero-padded on the left (RFC 8017 I2OSP).

</div>
</article>

<article class="doc-entry" id="v-rsa-key-bits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-key-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-key-bits" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L529" title="View source at line 529">Source</a></span></div>
<div class="doc-body">

Length of the RSA modulus in bits.

</div>
</article>

<article class="doc-entry" id="v-rsa-key-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-key-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L525" title="View source at line 525">Source</a></span></div>
<div class="doc-body">

Length of the RSA modulus in bytes.

</div>
</article>

<article class="doc-entry" id="v-rsa-marshal-pkcs1-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-marshal-pkcs1-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-marshal-pkcs1-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L588" title="View source at line 588">Source</a></span></div>
<div class="doc-body">

Marshal an RSA private key to PKCS#1 ASN.1 DER (RFC 8017 Appendix A.1.2).

</div>
</article>

<article class="doc-entry" id="v-rsa-marshal-pkcs1-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-marshal-pkcs1-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-marshal-pkcs1-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L571" title="View source at line 571">Source</a></span></div>
<div class="doc-body">

Marshal an RSA public key to PKCS#1 ASN.1 DER (RFC 8017 Appendix A.1.1).

</div>
</article>

<article class="doc-entry" id="v-rsa-os2ip">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-os2ip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-os2ip" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L232" title="View source at line 232">Source</a></span></div>
<div class="doc-body">

Convert an unsigned big-endian byte string to a Nat (RFC 8017 OS2IP).

</div>
</article>

<article class="doc-entry" id="v-rsa-parse-pkcs1-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-parse-pkcs1-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a></code><span class="doc-sig-links"><a href="#v-rsa-parse-pkcs1-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L604" title="View source at line 604">Source</a></span></div>
<div class="doc-body">

Parse a two-prime RSA private key from PKCS#1 ASN.1 DER.

</div>
</article>

<article class="doc-entry" id="v-rsa-parse-pkcs1-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-parse-pkcs1-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a></code><span class="doc-sig-links"><a href="#v-rsa-parse-pkcs1-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L576" title="View source at line 576">Source</a></span></div>
<div class="doc-body">

Parse an RSA public key from PKCS#1 ASN.1 DER.

</div>
</article>

<article class="doc-entry" id="v-rsa-private-d">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-private-d</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-private-d" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L501" title="View source at line 501">Source</a></span></div>
<div class="doc-body">

Private exponent d of an RSA private key.

</div>
</article>

<article class="doc-entry" id="v-rsa-private-dp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-private-dp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-private-dp" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L513" title="View source at line 513">Source</a></span></div>
<div class="doc-body">

Precomputed CRT exponent dp = d mod (p - 1).

</div>
</article>

<article class="doc-entry" id="v-rsa-private-dq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-private-dq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-private-dq" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L517" title="View source at line 517">Source</a></span></div>
<div class="doc-body">

Precomputed CRT exponent dq = d mod (q - 1).

</div>
</article>

<article class="doc-entry" id="v-rsa-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a></code><span class="doc-sig-links"><a href="#v-rsa-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L469" title="View source at line 469">Source</a></span></div>
<div class="doc-body">

Construct, validate and precompute CRT parameters for a two-prime RSA private key.

</div>
</article>

<article class="doc-entry" id="v-rsa-private-key-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-private-key-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-rsa-private-key-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L553" title="View source at line 553">Source</a></span></div>
<div class="doc-body">

Compare two RSA private keys for equality.

</div>
</article>

<article class="doc-entry" id="v-rsa-private-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-private-p</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-private-p" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L505" title="View source at line 505">Source</a></span></div>
<div class="doc-body">

First prime factor p of an RSA private key.

</div>
</article>

<article class="doc-entry" id="v-rsa-private-q">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-private-q</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-private-q" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L509" title="View source at line 509">Source</a></span></div>
<div class="doc-body">

Second prime factor q of an RSA private key.

</div>
</article>

<article class="doc-entry" id="v-rsa-private-qinv">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-private-qinv</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-private-qinv" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L521" title="View source at line 521">Source</a></span></div>
<div class="doc-body">

Precomputed CRT coefficient qinv = q^(-1) mod p.

</div>
</article>

<article class="doc-entry" id="v-rsa-public-e">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-public-e</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-public-e" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L497" title="View source at line 497">Source</a></span></div>
<div class="doc-body">

Public exponent e of an RSA public key.

</div>
</article>

<article class="doc-entry" id="v-rsa-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a></code><span class="doc-sig-links"><a href="#v-rsa-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L461" title="View source at line 461">Source</a></span></div>
<div class="doc-body">

Construct and validate an RSA public key from modulus n and public exponent e.

</div>
</article>

<article class="doc-entry" id="v-rsa-public-key-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-public-key-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-rsa-public-key-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L549" title="View source at line 549">Source</a></span></div>
<div class="doc-body">

Compare two RSA public keys for equality.

</div>
</article>

<article class="doc-entry" id="v-rsa-public-key-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-public-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a></code><span class="doc-sig-links"><a href="#v-rsa-public-key-of" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L489" title="View source at line 489">Source</a></span></div>
<div class="doc-body">

Extract the public key corresponding to an RSA private key.

</div>
</article>

<article class="doc-entry" id="v-rsa-public-n">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-public-n</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rsa-public-n" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L493" title="View source at line 493">Source</a></span></div>
<div class="doc-body">

Modulus n of an RSA public key.

</div>
</article>

<article class="doc-entry" id="v-rsa-sign-pkcs1v15">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-sign-pkcs1v15</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-sign-pkcs1v15" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L723" title="View source at line 723">Source</a></span></div>
<div class="doc-body">

Sign a pre-computed message digest with RSASSA-PKCS1-v1_5.

</div>
</article>

<article class="doc-entry" id="v-rsa-sign-pss">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-sign-pss</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-PssSaltLength" title="Crypto.Rsa.PssSaltLength">PssSaltLength</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-sign-pss" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L801" title="View source at line 801">Source</a></span></div>
<div class="doc-body">

Sign a pre-computed message digest with RSASSA-PSS, deriving salt bytes deterministically from seed.

</div>
</article>

<article class="doc-entry" id="v-rsa-sign-pss-deterministic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-sign-pss-deterministic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rsa-sign-pss-deterministic" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L790" title="View source at line 790">Source</a></span></div>
<div class="doc-body">

Sign a pre-computed message digest with RSASSA-PSS using explicit salt bytes.

</div>
</article>

<article class="doc-entry" id="v-rsa-validate-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-validate-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-rsa-validate-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L533" title="View source at line 533">Source</a></span></div>
<div class="doc-body">

Re-validate all structural and mathematical invariants of an RSA private key.

</div>
</article>

<article class="doc-entry" id="v-rsa-verify-pkcs1v15">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-verify-pkcs1v15</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-rsa-verify-pkcs1v15" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L733" title="View source at line 733">Source</a></span></div>
<div class="doc-body">

Verify an RSASSA-PKCS1-v1_5 signature over a pre-computed message digest.

</div>
</article>

<article class="doc-entry" id="v-rsa-verify-pss">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rsa-verify-pss</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaHash" title="Crypto.Rsa.RsaHash">RsaHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-PssSaltLength" title="Crypto.Rsa.PssSaltLength">PssSaltLength</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaError" title="Crypto.Rsa.RsaError">RsaError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-rsa-verify-pss" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rsa/source/#L831" title="View source at line 831">Source</a></span></div>
<div class="doc-body">

Verify an RSASSA-PSS signature over a pre-computed message digest.

</div>
</article>
