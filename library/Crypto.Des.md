# Crypto.Des

DES and Triple DES block ciphers with ECB, CBC and CTR for legacy interoperation, from a generated straight-line round kernel.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.Des</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.Des/source/">View source</a><a href="/packages/search/?q=%2BCrypto.Des">Search module</a><a href="/reference-sources/Crypto.Des.cinch" download="Crypto.Des.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Hex](Text.Hex.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

DES and Triple DES (FIPS 46-3, SP 800-67) block ciphers with ECB, CBC and CTR over 64-bit blocks.
These ciphers exist for interoperation with legacy formats and protocols: DES
keys are far too short, and Triple DES has a 64-bit block, so neither should
protect new data. A whole block is one straight-line kernel: permutation
networks for the initial and final permutations, rotation windows for the
expansion, and combined S-box and P-permutation tables addressed through the
key schedule. Table lookups depend on data, so timing is not constant. Triple
DES is encrypt-decrypt-encrypt with three independent keys (a 24-byte key).
Every returned buffer is fresh; inputs are never retained or mutated.

</details>

<details class="doc-synopsis" open><summary>Synopsis (13 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-DesCipher">DesCipher</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DesCipher</strong></code></li><li><a class="syn-name" href="#t-DesError">DesError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DesError</strong></code></li><li><a class="syn-name" href="#v-des-block-size">des-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">des-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-des-cbc-decrypt">des-cbc-decrypt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">des-cbc-decrypt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-des-cbc-encrypt">des-cbc-encrypt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">des-cbc-encrypt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-des-cipher">des-cipher</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">des-cipher</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a></code></li><li><a class="syn-name" href="#v-des-ctr-xor">des-ctr-xor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">des-ctr-xor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-des-decrypt-block">des-decrypt-block</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">des-decrypt-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-des-decrypt-ecb">des-decrypt-ecb</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">des-decrypt-ecb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-des-encrypt-block">des-encrypt-block</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">des-encrypt-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-des-encrypt-ecb">des-encrypt-ecb</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">des-encrypt-ecb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-des-error-message">des-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">des-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-triple-des-cipher">triple-des-cipher</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">triple-des-cipher</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-DesCipher">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DesCipher</strong></code><span class="doc-sig-links"><a href="#t-DesCipher" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

A keyed cipher: one schedule for DES, three for Triple DES (applied encrypt, decrypt, encrypt).
A schedule holds the SP tables (2048 bytes), the encryption round keys and the decryption round keys (128 bytes each).

</div>
</article>

<article class="doc-entry" id="t-DesError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DesError</strong></code><span class="doc-sig-links"><a href="#t-DesError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

A rejected key, input or initialization vector.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-DesInvalidKeySize"><code><strong class="tok-constructor">DesInvalidKeySize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a></code></li><li id="k-DesInvalidInputSize"><code><strong class="tok-constructor">DesInvalidInputSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a></code></li><li id="k-DesInvalidIvSize"><code><strong class="tok-constructor">DesInvalidIvSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-des-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">des-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-des-block-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

The block size in bytes.

</div>
</article>

<article class="doc-entry" id="v-des-cbc-decrypt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">des-cbc-decrypt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-des-cbc-decrypt" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L172" title="View source at line 172">Source</a></span></div>
<div class="doc-body">

CBC decryption.

</div>
</article>

<article class="doc-entry" id="v-des-cbc-encrypt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">des-cbc-encrypt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-des-cbc-encrypt" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

CBC encryption: each plaintext block is XORed with the previous ciphertext block (the IV first).

</div>
</article>

<article class="doc-entry" id="v-des-cipher">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">des-cipher</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a></code><span class="doc-sig-links"><a href="#v-des-cipher" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">

Key DES with an 8-byte key (parity bits are ignored).

</div>
</article>

<article class="doc-entry" id="v-des-ctr-xor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">des-ctr-xor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-des-ctr-xor" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L188" title="View source at line 188">Source</a></span></div>
<div class="doc-body">

CTR mode: XOR the input (any length) with encryptions of the counter block, which increments as a
big-endian 64-bit integer. Encryption and decryption are the same operation.

</div>
</article>

<article class="doc-entry" id="v-des-decrypt-block">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">des-decrypt-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-des-decrypt-block" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

Decrypt exactly one 8-byte block.

</div>
</article>

<article class="doc-entry" id="v-des-decrypt-ecb">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">des-decrypt-ecb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-des-decrypt-ecb" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

Decrypt each block independently.

</div>
</article>

<article class="doc-entry" id="v-des-encrypt-block">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">des-encrypt-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-des-encrypt-block" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

Encrypt exactly one 8-byte block.

</div>
</article>

<article class="doc-entry" id="v-des-encrypt-ecb">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">des-encrypt-ecb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-des-encrypt-ecb" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L136" title="View source at line 136">Source</a></span></div>
<div class="doc-body">

Encrypt each block independently.

</div>
</article>

<article class="doc-entry" id="v-des-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">des-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-des-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Render an error.

</div>
</article>

<article class="doc-entry" id="v-triple-des-cipher">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">triple-des-cipher</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesError" title="Crypto.Des.DesError">DesError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Des/#t-DesCipher" title="Crypto.Des.DesCipher">DesCipher</a></code><span class="doc-sig-links"><a href="#v-triple-des-cipher" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Des/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

Key Triple DES with a 24-byte key: three DES keys in order.

</div>
</article>
