# Crypto.Cipher

CTR, OFB, CFB, CBC and GCM over any 128-bit block cipher, with constant-time or carry-less-instruction GHASH and constant-time tag checks.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Crypto.Cipher</code></div><div class="doc-mod-bar-links"><a href="/library/source/Crypto.Cipher/">View source</a><a href="/library/search/?q=%2BCrypto.Cipher">Search module</a><a href="/reference-sources/Crypto.Cipher.cinch" download="Crypto.Cipher.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Crypto.Cipher`: [Crypto.Cipher](Crypto.Cipher.md), [Data.Bytes.Word](Data.Bytes.Word.md).

```sh
cp seed/prelude/Crypto/Cipher.cinch ./Crypto.Cipher.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Block-cipher modes over any 128-bit block cipher: CTR, OFB and CFB keystreams, CBC, and GCM authenticated encryption.
A BlockCipher packages a cipher's forward and inverse block functions over
U128 blocks whose byte i occupies bits 8i..8i+7 (memory order). Keystream
modes share one explicit state machine, XorStream, whose register, buffered
keystream and consumption count persist across calls of any length; retaining
a stream branches it. CBC processes whole blocks and carries its chaining
value explicitly. GCM follows NIST SP 800-38D: GHASH uses a constant-time
portable multiplication or PCLMULQDQ after CPU feature detection, tags are
compared in constant time, and no plaintext is released before the tag
verifies. Every returned buffer is fresh; inputs are never retained or mutated.

</details>

<details class="doc-synopsis"><summary>Synopsis (35 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-BlockCipher">BlockCipher</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BlockCipher</strong></code></li><li><a class="syn-name" href="#t-CbcState">CbcState</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CbcState</strong></code></li><li><a class="syn-name" href="#t-CipherError">CipherError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CipherError</strong></code></li><li><a class="syn-name" href="#t-Gcm">Gcm</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Gcm</strong></code></li><li><a class="syn-name" href="#t-XorStream">XorStream</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XorStream</strong></code></li><li><a class="syn-name" href="#v-block-cipher">block-cipher</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">block-cipher</strong> :: (<a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a></code></li><li><a class="syn-name" href="#v-block-decrypt">block-decrypt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">block-decrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-block-encrypt">block-encrypt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">block-encrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-block-from-bytes">block-from-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">block-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-block-to-bytes">block-to-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">block-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-cbc-crypt-blocks">cbc-crypt-blocks</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cbc-crypt-blocks</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CbcState" title="Crypto.Cipher.CbcState">CbcState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CbcState" title="Crypto.Cipher.CbcState">CbcState</a>)</code></li><li><a class="syn-name" href="#v-cbc-decrypt">cbc-decrypt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cbc-decrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-cbc-decrypter">cbc-decrypter</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cbc-decrypter</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CbcState" title="Crypto.Cipher.CbcState">CbcState</a></code></li><li><a class="syn-name" href="#v-cbc-encrypt">cbc-encrypt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cbc-encrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-cbc-encrypter">cbc-encrypter</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cbc-encrypter</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CbcState" title="Crypto.Cipher.CbcState">CbcState</a></code></li><li><a class="syn-name" href="#v-cfb-decrypt">cfb-decrypt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cfb-decrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-cfb-decrypt-stream">cfb-decrypt-stream</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cfb-decrypt-stream</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a></code></li><li><a class="syn-name" href="#v-cfb-encrypt">cfb-encrypt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cfb-encrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-cfb-encrypt-stream">cfb-encrypt-stream</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cfb-encrypt-stream</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a></code></li><li><a class="syn-name" href="#v-cipher-block-size">cipher-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cipher-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-cipher-error-message">cipher-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cipher-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-ctr-stream">ctr-stream</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ctr-stream</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a></code></li><li><a class="syn-name" href="#v-ctr-xor">ctr-xor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ctr-xor</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gcm">gcm</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gcm</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a></code></li><li><a class="syn-name" href="#v-gcm-hardware">gcm-hardware</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gcm-hardware</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-gcm-nonce-size">gcm-nonce-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gcm-nonce-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-gcm-open">gcm-open</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gcm-open</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gcm-overhead">gcm-overhead</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gcm-overhead</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-gcm-seal">gcm-seal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gcm-seal</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gcm-tag-size">gcm-tag-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gcm-tag-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-gcm-with-nonce-size">gcm-with-nonce-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gcm-with-nonce-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a></code></li><li><a class="syn-name" href="#v-gcm-with-tag-size">gcm-with-tag-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gcm-with-tag-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a></code></li><li><a class="syn-name" href="#v-ofb-stream">ofb-stream</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ofb-stream</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a></code></li><li><a class="syn-name" href="#v-ofb-xor">ofb-xor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ofb-xor</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-stream-xor">stream-xor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">stream-xor</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-BlockCipher">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BlockCipher</strong></code><span class="doc-sig-links"><a href="#t-BlockCipher" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Encrypt and decrypt functions over canonical U128 blocks.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-BlockCipher"><code><strong class="tok-constructor">BlockCipher</strong> :: (<a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-CbcState">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CbcState</strong></code><span class="doc-sig-links"><a href="#t-CbcState" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L244" title="View source at line 244">Source</a></span></div>
<div class="doc-body">

CBC state: cipher, direction (True encrypts) and the chaining value for the next block.

</div>
</article>

<article class="doc-entry" id="t-CipherError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CipherError</strong></code><span class="doc-sig-links"><a href="#t-CipherError" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

A cipher misuse or authentication failure; every operation reports one before producing output.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidIvSize"><code><strong class="tok-constructor">InvalidIvSize</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a></code></li><li id="k-InvalidInputSize"><code><strong class="tok-constructor">InvalidInputSize</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a></code></li><li id="k-InvalidNonceSize"><code><strong class="tok-constructor">InvalidNonceSize</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a></code></li><li id="k-InvalidTagSize"><code><strong class="tok-constructor">InvalidTagSize</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a></code></li><li id="k-MessageTooLarge"><code><strong class="tok-constructor">MessageTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a></code></li><li id="k-AuthenticationFailed"><code><strong class="tok-constructor">AuthenticationFailed</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Gcm">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Gcm</strong></code><span class="doc-sig-links"><a href="#t-Gcm" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L292" title="View source at line 292">Source</a></span></div>
<div class="doc-body">

A GCM instance: cipher, hash key H, nonce size, tag size and whether GHASH uses carry-less instructions.

</div>
</article>

<article class="doc-entry" id="t-XorStream">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XorStream</strong></code><span class="doc-sig-links"><a href="#t-XorStream" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

A keystream mode's state: cipher, mode (1 CTR, 2 OFB, 3 CFB encrypt, 4 CFB decrypt), register,
current keystream block, bytes of it already used (0 to 16), and the CFB ciphertext feedback gathered so far.

</div>
</article>

# Values

<article class="doc-entry" id="v-block-cipher">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">block-cipher</strong> :: (<a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a></code><span class="doc-sig-links"><a href="#v-block-cipher" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

Package forward and inverse block functions; decrypt must invert encrypt.

</div>
</article>

<article class="doc-entry" id="v-block-decrypt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">block-decrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-block-decrypt" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

Apply the inverse permutation to one block.

</div>
</article>

<article class="doc-entry" id="v-block-encrypt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">block-encrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-block-encrypt" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Apply the forward permutation to one block.

</div>
</article>

<article class="doc-entry" id="v-block-from-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">block-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-block-from-bytes" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Read sixteen bytes at a position as a canonical block, or None when they are not all present.

</div>
</article>

<article class="doc-entry" id="v-block-to-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">block-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-block-to-bytes" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

Return a block's sixteen bytes in fresh storage.

</div>
</article>

<article class="doc-entry" id="v-cbc-crypt-blocks">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cbc-crypt-blocks</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CbcState" title="Crypto.Cipher.CbcState">CbcState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CbcState" title="Crypto.Cipher.CbcState">CbcState</a>)</code><span class="doc-sig-links"><a href="#v-cbc-crypt-blocks" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L258" title="View source at line 258">Source</a></span></div>
<div class="doc-body">

Encrypt or decrypt whole blocks, returning fresh output and the state chained to the last block.
Input that is not a multiple of sixteen bytes is rejected before any work; CBC applies no padding.

</div>
</article>

<article class="doc-entry" id="v-cbc-decrypt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cbc-decrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-cbc-decrypt" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L286" title="View source at line 286">Source</a></span></div>
<div class="doc-body">

One-shot CBC decryption of whole blocks.

</div>
</article>

<article class="doc-entry" id="v-cbc-decrypter">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cbc-decrypter</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CbcState" title="Crypto.Cipher.CbcState">CbcState</a></code><span class="doc-sig-links"><a href="#v-cbc-decrypter" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L253" title="View source at line 253">Source</a></span></div>
<div class="doc-body">

Start CBC decryption from a sixteen-byte IV.

</div>
</article>

<article class="doc-entry" id="v-cbc-encrypt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cbc-encrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-cbc-encrypt" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L280" title="View source at line 280">Source</a></span></div>
<div class="doc-body">

One-shot CBC encryption of whole blocks.

</div>
</article>

<article class="doc-entry" id="v-cbc-encrypter">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cbc-encrypter</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CbcState" title="Crypto.Cipher.CbcState">CbcState</a></code><span class="doc-sig-links"><a href="#v-cbc-encrypter" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L249" title="View source at line 249">Source</a></span></div>
<div class="doc-body">

Start CBC encryption from a sixteen-byte IV.

</div>
</article>

<article class="doc-entry" id="v-cfb-decrypt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cfb-decrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-cfb-decrypt" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L238" title="View source at line 238">Source</a></span></div>
<div class="doc-body">

One-shot cipher-feedback decryption from an IV.

</div>
</article>

<article class="doc-entry" id="v-cfb-decrypt-stream">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cfb-decrypt-stream</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a></code><span class="doc-sig-links"><a href="#v-cfb-decrypt-stream" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L149" title="View source at line 149">Source</a></span></div>
<div class="doc-body">

Full-block cipher feedback decryption, the inverse of cfb-encrypt-stream for the same IV.

</div>
</article>

<article class="doc-entry" id="v-cfb-encrypt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cfb-encrypt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-cfb-encrypt" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L232" title="View source at line 232">Source</a></span></div>
<div class="doc-body">

One-shot cipher-feedback encryption from an IV.

</div>
</article>

<article class="doc-entry" id="v-cfb-encrypt-stream">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cfb-encrypt-stream</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a></code><span class="doc-sig-links"><a href="#v-cfb-encrypt-stream" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L145" title="View source at line 145">Source</a></span></div>
<div class="doc-body">

Full-block cipher feedback encryption: each keystream block encrypts the previous ciphertext block.

</div>
</article>

<article class="doc-entry" id="v-cipher-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cipher-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-cipher-block-size" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

Block size in bytes, always 16.

</div>
</article>

<article class="doc-entry" id="v-cipher-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cipher-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cipher-error-message" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

Render a cipher error.

</div>
</article>

<article class="doc-entry" id="v-ctr-stream">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ctr-stream</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a></code><span class="doc-sig-links"><a href="#v-ctr-stream" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

Counter mode: the IV is a 128-bit big-endian counter incremented once per block, wrapping.

</div>
</article>

<article class="doc-entry" id="v-ctr-xor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ctr-xor</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ctr-xor" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L220" title="View source at line 220">Source</a></span></div>
<div class="doc-body">

One-shot counter-mode XOR from an IV.

</div>
</article>

<article class="doc-entry" id="v-gcm">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gcm</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a></code><span class="doc-sig-links"><a href="#v-gcm" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L297" title="View source at line 297">Source</a></span></div>
<div class="doc-body">

GCM with the standard 12-byte nonce and 16-byte tag.

</div>
</article>

<article class="doc-entry" id="v-gcm-hardware">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gcm-hardware</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-gcm-hardware" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L324" title="View source at line 324">Source</a></span></div>
<div class="doc-body">

Whether GHASH runs on carry-less multiplication instructions.

</div>
</article>

<article class="doc-entry" id="v-gcm-nonce-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gcm-nonce-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-gcm-nonce-size" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L312" title="View source at line 312">Source</a></span></div>
<div class="doc-body">

Required nonce size in bytes.

</div>
</article>

<article class="doc-entry" id="v-gcm-open">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gcm-open</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gcm-open" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L403" title="View source at line 403">Source</a></span></div>
<div class="doc-body">

Verify and decrypt ciphertext followed by its tag. Verification is constant time, and
AuthenticationFailed is returned without any plaintext when the tag does not match.

</div>
</article>

<article class="doc-entry" id="v-gcm-overhead">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gcm-overhead</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-gcm-overhead" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L320" title="View source at line 320">Source</a></span></div>
<div class="doc-body">

Bytes a sealed message adds to its plaintext: the tag size.

</div>
</article>

<article class="doc-entry" id="v-gcm-seal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gcm-seal</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gcm-seal" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L386" title="View source at line 386">Source</a></span></div>
<div class="doc-body">

Encrypt and authenticate: returns ciphertext followed by the tag in fresh storage.
Nonces must never repeat under one key; repetition destroys both confidentiality and authenticity.

</div>
</article>

<article class="doc-entry" id="v-gcm-tag-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gcm-tag-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-gcm-tag-size" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L316" title="View source at line 316">Source</a></span></div>
<div class="doc-body">

Tag size in bytes.

</div>
</article>

<article class="doc-entry" id="v-gcm-with-nonce-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gcm-with-nonce-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a></code><span class="doc-sig-links"><a href="#v-gcm-with-nonce-size" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L301" title="View source at line 301">Source</a></span></div>
<div class="doc-body">

GCM with a non-standard nonce size; sizes other than 12 derive the initial counter through GHASH.

</div>
</article>

<article class="doc-entry" id="v-gcm-with-tag-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gcm-with-tag-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-Gcm" title="Crypto.Cipher.Gcm">Gcm</a></code><span class="doc-sig-links"><a href="#v-gcm-with-tag-size" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L306" title="View source at line 306">Source</a></span></div>
<div class="doc-body">

GCM with a truncated tag of 12 to 16 bytes.

</div>
</article>

<article class="doc-entry" id="v-ofb-stream">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ofb-stream</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a></code><span class="doc-sig-links"><a href="#v-ofb-stream" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

Output feedback mode: each keystream block encrypts the previous one, starting from the IV.

</div>
</article>

<article class="doc-entry" id="v-ofb-xor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ofb-xor</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-CipherError" title="Crypto.Cipher.CipherError">CipherError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ofb-xor" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L226" title="View source at line 226">Source</a></span></div>
<div class="doc-body">

One-shot output-feedback XOR from an IV.

</div>
</article>

<article class="doc-entry" id="v-stream-xor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">stream-xor</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Crypto.Cipher/#t-XorStream" title="Crypto.Cipher.XorStream">XorStream</a>)</code><span class="doc-sig-links"><a href="#v-stream-xor" title="Permalink">#</a><a href="/library/source/Crypto.Cipher/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

XOR an input of any length with the keystream, returning fresh output and the advanced stream.
Applying the same stream to the output reverses it for CTR and OFB; CFB uses its matching decrypt stream.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Crypto/Cipher.cinch`](/library/source/Crypto.Cipher/). Source SHA-256: `229b114e7a3c8d8c97c9c91cd506b55f412dcdc018f9692d6540f8dc5cd30783`.
