# Crypto.Aes

AES-128/192/256 with AES-NI or ARMv8 instructions after detection and a constant-time, table-free SIMD fallback.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.Aes</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.Aes/source/">View source</a><a href="/packages/search/?q=%2BCrypto.Aes">Search module</a><a href="/reference-sources/Crypto.Aes.cinch" download="Crypto.Aes.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.Cipher](Crypto.Cipher.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

<details class="doc-notes"><summary>Module notes</summary>

AES-128, AES-192 and AES-256 block encryption with hardware instructions and a constant-time portable fallback.
Keys are expanded once into immutable round-key schedules. On x86-64 the
AES-NI instructions are used after CPUID detection; on AArch64 the ARMv8
cryptography extension is used when the target profile supplies ArmAes
evidence. Otherwise every round runs on all sixteen state bytes in SIMD lanes
with SubBytes computed arithmetically (inversion in GF(2^8) and the affine
map), so the fallback performs no table lookups and no data-dependent
branches. Blocks are U128 values with byte i in bits 8i..8i+7; byte-buffer
operations return fresh storage. aes-cipher adapts a key to Crypto.Cipher's
CTR, CBC, OFB, CFB and GCM modes.

</details>

<details class="doc-synopsis" open><summary>Synopsis (14 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-AesKey">AesKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AesKey</strong></code></li><li><a class="syn-name" href="#t-AesKeyError">AesKeyError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AesKeyError</strong></code></li><li><a class="syn-name" href="#c-ArmAes">ArmAes</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">ArmAes</strong></code></li><li><a class="syn-name" href="#v-aes-block-size">aes-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-aes-cipher">aes-cipher</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-cipher</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a></code></li><li><a class="syn-name" href="#v-aes-decrypt-block">aes-decrypt-block</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-decrypt-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-aes-decrypt-word">aes-decrypt-word</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-decrypt-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-aes-encrypt-block">aes-encrypt-block</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-encrypt-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-aes-encrypt-word">aes-encrypt-word</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-encrypt-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-aes-key">aes-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a></code></li><li><a class="syn-name" href="#v-aes-key-error-message">aes-key-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-key-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-aes-key-hardware">aes-key-hardware</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-key-hardware</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-aes-key-size">aes-key-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-aes-portable-key">aes-portable-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-portable-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-AesKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AesKey</strong></code><span class="doc-sig-links"><a href="#t-AesKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

An expanded key: key size in bytes, whether hardware rounds are used, and encryption and decryption schedules.
With hardware rounds the decryption schedule holds the equivalent inverse cipher's keys.

</div>
</article>

<article class="doc-entry" id="t-AesKeyError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AesKeyError</strong></code><span class="doc-sig-links"><a href="#t-AesKeyError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

A key or block of the wrong length.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidAesKeySize"><code><strong class="tok-constructor">InvalidAesKeySize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a></code></li><li id="k-InvalidAesBlockSize"><code><strong class="tok-constructor">InvalidAesBlockSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a></code></li></ul></div>
</div>
</article>

# Signatures

<article class="doc-entry" id="c-ArmAes">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">ArmAes</strong></code><span class="doc-sig-links"><a href="#c-ArmAes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

Target-profile evidence that the ARMv8 AES instructions may execute.
Only profiles that guarantee the cryptography extension should name this capability.

</div>
</article>

# Values

<article class="doc-entry" id="v-aes-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-aes-block-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

AES block size in bytes, always 16.

</div>
</article>

<article class="doc-entry" id="v-aes-cipher">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-cipher</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Cipher/#t-BlockCipher" title="Crypto.Cipher.BlockCipher">BlockCipher</a></code><span class="doc-sig-links"><a href="#v-aes-cipher" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L209" title="View source at line 209">Source</a></span></div>
<div class="doc-body">

Adapt a key to the generic block-cipher modes.

</div>
</article>

<article class="doc-entry" id="v-aes-decrypt-block">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-decrypt-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-aes-decrypt-block" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L203" title="View source at line 203">Source</a></span></div>
<div class="doc-body">

Decrypt exactly sixteen bytes into fresh storage.

</div>
</article>

<article class="doc-entry" id="v-aes-decrypt-word">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-decrypt-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-aes-decrypt-word" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

Decrypt one canonical block.

</div>
</article>

<article class="doc-entry" id="v-aes-encrypt-block">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-encrypt-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-aes-encrypt-block" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L197" title="View source at line 197">Source</a></span></div>
<div class="doc-body">

Encrypt exactly sixteen bytes into fresh storage.

</div>
</article>

<article class="doc-entry" id="v-aes-encrypt-word">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-encrypt-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-aes-encrypt-word" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Encrypt one canonical block.

</div>
</article>

<article class="doc-entry" id="v-aes-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a></code><span class="doc-sig-links"><a href="#v-aes-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Expand a 16-, 24- or 32-byte key, selecting hardware rounds when the processor supports them.

</div>
</article>

<article class="doc-entry" id="v-aes-key-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-key-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-aes-key-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Render a key or block size error.

</div>
</article>

<article class="doc-entry" id="v-aes-key-hardware">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-key-hardware</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-aes-key-hardware" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

Whether this key's rounds run on hardware AES instructions.

</div>
</article>

<article class="doc-entry" id="v-aes-key-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-aes-key-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

Key size in bytes: 16, 24 or 32.

</div>
</article>

<article class="doc-entry" id="v-aes-portable-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-portable-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKeyError" title="Crypto.Aes.AesKeyError">AesKeyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Aes/#t-AesKey" title="Crypto.Aes.AesKey">AesKey</a></code><span class="doc-sig-links"><a href="#v-aes-portable-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Aes/source/#L71" title="View source at line 71">Source</a></span></div>
<div class="doc-body">

Expand a key that always uses the constant-time portable rounds, whatever the processor supports.

</div>
</article>
