# Crypto.ChaCha20Poly1305

ChaCha20 and XChaCha20 streams, Poly1305 and the ChaCha20-Poly1305 and XChaCha20-Poly1305 AEADs with vector row kernels.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Crypto.ChaCha20Poly1305</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/">View source</a><a href="/packages/search/?q=%2BCrypto.ChaCha20Poly1305">Search module</a><a href="/reference-sources/Crypto.ChaCha20Poly1305.cinch" download="Crypto.ChaCha20Poly1305.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

<details class="doc-closure"><summary>Ship-source closure (3 files)</summary>

Copy these modules beside your entry file before importing `Crypto.ChaCha20Poly1305`: [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes.Word](Data.Bytes.Word.md).

```sh
cp seed/prelude/Crypto/ChaCha20Poly1305.cinch ./Crypto.ChaCha20Poly1305.cinch
cp seed/prelude/Crypto/ConstantTime.cinch ./Crypto.ConstantTime.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

ChaCha20 and XChaCha20 stream encryption, the Poly1305 one-time authenticator, and the
ChaCha20-Poly1305 and XChaCha20-Poly1305 authenticated ciphers (RFC 8439 and the extended-nonce
construction through HChaCha20).

A ChaCha20 keystream block is the state after twenty rounds plus the initial state, computed as four
<4 x i32> rows: column rounds are vector quarter rounds and diagonal rounds rotate rows one, two and three
lanes around them. Each kernel returns one sixteen-byte row, so a 64-byte block takes four calls that each
run the rounds. Poly1305 keeps its accumulator below 2^131 and multiplies with exact 256-bit products,
reducing modulo 2^130 - 5 once per block and fully at the end, with no data-dependent branches. Tags are
compared in constant time and nothing is decrypted before the tag verifies. Every returned buffer is fresh.

</details>

<details class="doc-synopsis" open><summary>Synopsis (15 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ChaChaError">ChaChaError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ChaChaError</strong></code></li><li><a class="syn-name" href="#v-chacha-error-message">chacha-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-chacha-key-size">chacha-key-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-chacha-nonce-size">chacha-nonce-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha-nonce-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-chacha20-xor">chacha20-xor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha20-xor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-chacha20poly1305-open">chacha20poly1305-open</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha20poly1305-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-chacha20poly1305-seal">chacha20poly1305-seal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha20poly1305-seal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hchacha20">hchacha20</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hchacha20</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-poly1305-mac">poly1305-mac</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">poly1305-mac</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-poly1305-tag-size">poly1305-tag-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">poly1305-tag-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-poly1305-verify">poly1305-verify</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">poly1305-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-xchacha-nonce-size">xchacha-nonce-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xchacha-nonce-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-xchacha20-xor">xchacha20-xor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xchacha20-xor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-xchacha20poly1305-open">xchacha20poly1305-open</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xchacha20poly1305-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-xchacha20poly1305-seal">xchacha20poly1305-seal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xchacha20poly1305-seal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ChaChaError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ChaChaError</strong></code><span class="doc-sig-links"><a href="#t-ChaChaError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

Why a ChaCha20 or Poly1305 operation was refused.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ChaChaKeySize"><code><strong class="tok-constructor">ChaChaKeySize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a></code></li><li id="k-ChaChaNonceSize"><code><strong class="tok-constructor">ChaChaNonceSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a></code></li><li id="k-ChaChaTooLarge"><code><strong class="tok-constructor">ChaChaTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a></code></li><li id="k-ChaChaAuthentication"><code><strong class="tok-constructor">ChaChaAuthentication</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-chacha-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-chacha-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

A readable description of an error.

</div>
</article>

<article class="doc-entry" id="v-chacha-key-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-chacha-key-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

Key bytes.

</div>
</article>

<article class="doc-entry" id="v-chacha-nonce-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha-nonce-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-chacha-nonce-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

ChaCha20 and ChaCha20-Poly1305 nonce bytes.

</div>
</article>

<article class="doc-entry" id="v-chacha20-xor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha20-xor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-chacha20-xor" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

XOR a message with the ChaCha20 keystream for a 32-byte key and 12-byte nonce, starting at a block counter.

</div>
</article>

<article class="doc-entry" id="v-chacha20poly1305-open">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha20poly1305-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-chacha20poly1305-open" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L278" title="View source at line 278">Source</a></span></div>
<div class="doc-body">

Verify and decrypt a sealed message; nothing is decrypted unless the tag verifies.

</div>
</article>

<article class="doc-entry" id="v-chacha20poly1305-seal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha20poly1305-seal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-chacha20poly1305-seal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L261" title="View source at line 261">Source</a></span></div>
<div class="doc-body">

Encrypt and authenticate: the ciphertext followed by the sixteen-byte tag.

</div>
</article>

<article class="doc-entry" id="v-hchacha20">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hchacha20</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hchacha20" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

HChaCha20 of a 32-byte key and the first 16 bytes of a nonce: rows 0 and 3 of the state after twenty
rounds without the final addition, as a 32-byte subkey.

</div>
</article>

<article class="doc-entry" id="v-poly1305-mac">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">poly1305-mac</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-poly1305-mac" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

The Poly1305 tag of a message under a 32-byte one-time key.

</div>
</article>

<article class="doc-entry" id="v-poly1305-tag-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">poly1305-tag-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-poly1305-tag-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

Poly1305 tag bytes.

</div>
</article>

<article class="doc-entry" id="v-poly1305-verify">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">poly1305-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-poly1305-verify" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L207" title="View source at line 207">Source</a></span></div>
<div class="doc-body">

Whether a tag authenticates a message under a one-time key, compared in constant time.

</div>
</article>

<article class="doc-entry" id="v-xchacha-nonce-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xchacha-nonce-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-xchacha-nonce-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">

XChaCha20 and XChaCha20-Poly1305 nonce bytes.

</div>
</article>

<article class="doc-entry" id="v-xchacha20-xor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xchacha20-xor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-xchacha20-xor" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L186" title="View source at line 186">Source</a></span></div>
<div class="doc-body">

XOR with the XChaCha20 keystream of a 32-byte key and 24-byte nonce: ChaCha20 under the HChaCha20 subkey of
the first sixteen nonce bytes, with four zero bytes and the last eight nonce bytes as its nonce.

</div>
</article>

<article class="doc-entry" id="v-xchacha20poly1305-open">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xchacha20poly1305-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-xchacha20poly1305-open" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L301" title="View source at line 301">Source</a></span></div>
<div class="doc-body">

XChaCha20-Poly1305 opening with a 24-byte nonce.

</div>
</article>

<article class="doc-entry" id="v-xchacha20poly1305-seal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xchacha20poly1305-seal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ChaCha20Poly1305/#t-ChaChaError" title="Crypto.ChaCha20Poly1305.ChaChaError">ChaChaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-xchacha20poly1305-seal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ChaCha20Poly1305/source/#L294" title="View source at line 294">Source</a></span></div>
<div class="doc-body">

XChaCha20-Poly1305 sealing with a 24-byte nonce.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Crypto/ChaCha20Poly1305.cinch`](/packages/stdlib/Crypto.ChaCha20Poly1305/source/). Source SHA-256: `196fddc50996e7438fb32c0c994df4a1201f453f85d0669293a933a2d04db699`.
