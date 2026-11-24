# Text.Checksum

Incremental Adler-32, CRC-32 and CRC-64 checksums with explicit reflected polynomials.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Checksum</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Checksum/source/">View source</a><a href="/packages/search/?q=%2BText.Checksum">Search module</a><a href="/reference-sources/Text.Checksum.cinch" download="Text.Checksum.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Text.Adler32](Text.Adler32.md), [Text.Crc64](Text.Crc64.md).

<details class="doc-notes"><summary>Module notes</summary>

Incremental Adler-32, CRC-32 and CRC-64 checksums over byte buffers.
Updates accept finished checksums: updating the checksum of a prefix with a
suffix equals checksumming their concatenation. Start Adler-32 at 1 and CRCs
at 0. Polynomial arguments use reflected representations. Text.Adler32 supplies
the portable SIMD accumulator; CRCs consume native eight-byte blocks.
CRC-32C uses an ISA instruction after CPU feature detection on x86. AArch64
profiles with ArmCrc32 accelerate both IEEE and Castagnoli. Other cases use portable
inline LLVM shift registers. These checksums do not authenticate messages.

</details>

<details class="doc-synopsis" open><summary>Synopsis (13 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-ArmCrc32">ArmCrc32</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">ArmCrc32</strong></code></li><li><a class="syn-name" href="#v-adler32-sum">adler32-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-adler32-update">adler32-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc32-castagnoli">crc32-castagnoli</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-castagnoli</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc32-ieee">crc32-ieee</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-ieee</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc32-koopman">crc32-koopman</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-koopman</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc32-sum">crc32-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc32-update">crc32-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc32-with">crc32-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc64-ecma">crc64-ecma</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-ecma</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-crc64-iso">crc64-iso</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-iso</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-crc64-sum">crc64-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-crc64-update">crc64-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-ArmCrc32">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">ArmCrc32</strong></code><span class="doc-sig-links"><a href="#c-ArmCrc32" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

Target-profile evidence that AArch64 CRC instructions may execute.
Only profiles that guarantee the CRC extension should name this capability.

</div>
</article>

# Values

<article class="doc-entry" id="v-adler32-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-adler32-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Compute Adler-32 over all input bytes, starting at the standard state 1. The empty checksum is 1.

</div>
</article>

<article class="doc-entry" id="v-adler32-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-adler32-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

Continue a finished Adler-32 checksum with another chunk. Empty input preserves the supplied state.

</div>
</article>

<article class="doc-entry" id="v-crc32-castagnoli">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-castagnoli</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32-castagnoli" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L43" title="View source at line 43">Source</a></span></div>
<div class="doc-body">

The reflected Castagnoli CRC-32C polynomial, 0x82F63B78.

</div>
</article>

<article class="doc-entry" id="v-crc32-ieee">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-ieee</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32-ieee" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

The reflected IEEE CRC-32 polynomial, 0xEDB88320.

</div>
</article>

<article class="doc-entry" id="v-crc32-koopman">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-koopman</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32-koopman" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

The reflected Koopman CRC-32 polynomial, 0xEB31D82E.

</div>
</article>

<article class="doc-entry" id="v-crc32-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Compute the IEEE CRC-32 checksum. The empty checksum is zero.

</div>
</article>

<article class="doc-entry" id="v-crc32-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Continue a finished CRC-32 with another chunk. Arguments are reflected polynomial, previous checksum, and bytes; initial/final complementing is internal.

</div>
</article>

<article class="doc-entry" id="v-crc32-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32-with" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

Compute CRC-32 with the supplied reflected polynomial, starting from the finished state zero.

</div>
</article>

<article class="doc-entry" id="v-crc64-ecma">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-ecma</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-ecma" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

The reflected ECMA CRC-64 polynomial, 0xC96C5795D7870F42.

</div>
</article>

<article class="doc-entry" id="v-crc64-iso">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-iso</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-iso" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

The reflected ISO CRC-64 polynomial, 0xD800000000000000.

</div>
</article>

<article class="doc-entry" id="v-crc64-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

Compute CRC-64 with a supplied reflected polynomial. Empty input yields zero.
Reuse a prepared Text.Crc64 table when processing many chunks of one polynomial.

</div>
</article>

<article class="doc-entry" id="v-crc64-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Checksum/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

Continue a finished CRC-64 with the same reflected polynomial and additional bytes.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Checksum.cinch`](/packages/stdlib/Text.Checksum/source/). Source SHA-256: `6f645a7b6fd84e7edbd8b66d0ec3bf721621deb409d3f89affddac9df7f4f684`.
