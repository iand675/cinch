# Crypto.Kdf

HKDF and PBKDF2 key derivation with checked lengths, binary inputs and HMAC-SHA-256.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.Kdf</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.Kdf/source/">View source</a><a href="/packages/search/?q=%2BCrypto.Kdf">Search module</a><a href="/reference-sources/Crypto.Kdf.cinch" download="Crypto.Kdf.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Text.Digest](Text.Digest.md).

<details class="doc-notes"><summary>Module notes</summary>

HKDF (RFC 5869) and PBKDF2 (RFC 8018) over a fixed-width keyed PRF.
The supplied default is HMAC-SHA-256. Passwords, keys, salt and info are
arbitrary bytes. Intermediate and returned buffers use ordinary Bytes storage.

</details>

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-KdfError">KdfError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">KdfError</strong></code></li><li><a class="syn-name" href="#t-KdfPrf">KdfPrf</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">KdfPrf</strong></code></li><li><a class="syn-name" href="#v-hkdf-expand">hkdf-expand</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hkdf-expand</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hkdf-extract">hkdf-extract</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hkdf-extract</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hkdf-key">hkdf-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hkdf-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-kdf-hmac-sha256">kdf-hmac-sha256</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">kdf-hmac-sha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a></code></li><li><a class="syn-name" href="#v-kdf-prf">kdf-prf</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">kdf-prf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a></code></li><li><a class="syn-name" href="#v-pbkdf2-key">pbkdf2-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pbkdf2-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-KdfError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">KdfError</strong></code><span class="doc-sig-links"><a href="#t-KdfError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Kdf/source/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Parameter and callback failures. No partially derived key is returned.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-KdfInvalidLength"><code><strong class="tok-constructor">KdfInvalidLength</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a></code></li><li id="k-KdfInvalidIterations"><code><strong class="tok-constructor">KdfInvalidIterations</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a></code></li><li id="k-KdfShortKey"><code><strong class="tok-constructor">KdfShortKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a></code></li><li id="k-KdfDigestSize"><code><strong class="tok-constructor">KdfDigestSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-KdfPrf">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">KdfPrf</strong></code><span class="doc-sig-links"><a href="#t-KdfPrf" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Kdf/source/#L18" title="View source at line 18">Source</a></span></div>
<div class="doc-body">

A deterministic keyed function with a nonzero, fixed output width.

</div>
</article>

# Values

<article class="doc-entry" id="v-hkdf-expand">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hkdf-expand</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hkdf-expand" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Kdf/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

Expand an extracted key using arbitrary context bytes. Output is limited
to 255 digest blocks; a zero-byte request returns a fresh empty buffer.

</div>
</article>

<article class="doc-entry" id="v-hkdf-extract">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hkdf-extract</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hkdf-extract" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Kdf/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

Extract a pseudorandom key from secret input material. An empty salt
selects the RFC default of one digest-width block of zero bytes.

</div>
</article>

<article class="doc-entry" id="v-hkdf-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hkdf-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hkdf-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Kdf/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Extract then expand. Reject excessive output before invoking the PRF.

</div>
</article>

<article class="doc-entry" id="v-kdf-hmac-sha256">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">kdf-hmac-sha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a></code><span class="doc-sig-links"><a href="#v-kdf-hmac-sha256" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Kdf/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

The runtime's RFC 2104 HMAC with SHA-256.

</div>
</article>

<article class="doc-entry" id="v-kdf-prf">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">kdf-prf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a></code><span class="doc-sig-links"><a href="#v-kdf-prf" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Kdf/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

The callback takes key then message and must return a fresh digest while
preserving its inputs. A zero output width is rejected at construction.

</div>
</article>

<article class="doc-entry" id="v-pbkdf2-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pbkdf2-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfPrf" title="Crypto.Kdf.KdfPrf">KdfPrf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Kdf/#t-KdfError" title="Crypto.Kdf.KdfError">KdfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-pbkdf2-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Kdf/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Derive a positive-length key with at least one iteration. At most
2^32-1 digest blocks can be numbered by PBKDF2's big-endian counter.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Crypto/Kdf.cinch`](/packages/stdlib/Crypto.Kdf/source/). Source SHA-256: `ee877d5b4b05c5425637d29b05dd35b3012f291e16bb67d159fe820484033291`.
