# Data.Bytes.Word

Bounds-proved unaligned little-endian machine-word reads and writes for SIMD and SWAR processing.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Bytes.Word</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Bytes.Word/source/">View source</a><a href="/packages/search/?q=%2BData.Bytes.Word">Search module</a><a href="/reference-sources/Data.Bytes.Word.cinch" download="Data.Bytes.Word.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md).

<details class="doc-notes"><summary>Module notes</summary>

Unaligned little-endian machine-word access to byte buffers.
Bounds are explicit preconditions in the public signatures. Reads and writes
access exactly the proved byte range via Data.Bytes word intrinsics; raw
pointers never enter the public interface. The packed words feed SIMD and
SWAR kernels. Wire order is independent of host endianness. Concurrent
mutation follows the same rules as Data.Bytes.

</details>

<details class="doc-synopsis" open><summary>Synopsis (4 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-bytes-read-le32">bytes-read-le32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-read-le32</strong> :: <span class="tok-variable">arg1</span> + 4 &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-bytes-read-le64">bytes-read-le64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-read-le64</strong> :: <span class="tok-variable">arg1</span> + 8 &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-bytes-write-le32">bytes-write-le32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-write-le32</strong> :: <span class="tok-variable">arg1</span> + 4 &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-bytes-write-le64">bytes-write-le64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-write-le64</strong> :: <span class="tok-variable">arg1</span> + 8 &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-bytes-read-le32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-read-le32</strong> :: <span class="tok-variable">arg1</span> + 4 &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-bytes-read-le32" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Word/source/#L16" title="View source at line 16">Source</a></span></div>
<div class="doc-body">

Read exactly 4 bytes as a little-endian U32. Alignment is unrestricted. The caller must prove pos + 4 <= bytes-len b; no tail bytes are read.

</div>
</article>

<article class="doc-entry" id="v-bytes-read-le64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-read-le64</strong> :: <span class="tok-variable">arg1</span> + 8 &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-bytes-read-le64" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Word/source/#L24" title="View source at line 24">Source</a></span></div>
<div class="doc-body">

Read exactly 8 bytes as a little-endian U64. Alignment is unrestricted. The caller must prove pos + 8 <= bytes-len b; no tail bytes are read.

</div>
</article>

<article class="doc-entry" id="v-bytes-write-le32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-write-le32</strong> :: <span class="tok-variable">arg1</span> + 4 &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-bytes-write-le32" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Word/source/#L20" title="View source at line 20">Source</a></span></div>
<div class="doc-body">

Write exactly 4 low-to-high bytes of a U32. The caller must prove pos + 4 <= bytes-len b; bytes outside that range are preserved.

</div>
</article>

<article class="doc-entry" id="v-bytes-write-le64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-write-le64</strong> :: <span class="tok-variable">arg1</span> + 8 &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-bytes-write-le64" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Word/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

Write exactly 8 low-to-high bytes of a U64. The caller must prove pos + 8 <= bytes-len b; bytes outside that range are preserved.

</div>
</article>
