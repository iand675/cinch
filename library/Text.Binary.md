# Text.Binary

Little- and big-endian integer encoding, checked buffer access, and signed/unsigned varints.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Binary</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Binary/source/">View source</a><a href="/packages/search/?q=%2BText.Binary">Search module</a><a href="/reference-sources/Text.Binary.cinch" download="Text.Binary.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Text.Binary`: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Unsigned fixed-width integers and signed/unsigned variable-length wire encodings.
Reads accept zero-based byte offsets and return named errors for short input.
Varint reads also return the number of consumed bytes, excluding the offset.
Encoders allocate fresh buffers; put operations check the complete destination
range before writing. Signed varints use zigzag encoding across the full I64
range. Fixed-width words use inline LLVM byte swaps; varint prefix detection
and payload compaction use SWAR, with an exact scalar tail.

</details>

<details class="doc-synopsis" open><summary>Synopsis (15 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-BinaryError">BinaryError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BinaryError</strong></code></li><li><a class="syn-name" href="#t-ByteOrder">ByteOrder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteOrder</strong></code></li><li><a class="syn-name" href="#v-binary-put-u16">binary-put-u16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-put-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-binary-put-u32">binary-put-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-put-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-binary-put-u64">binary-put-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-put-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-binary-read-u16">binary-read-u16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-read-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-binary-read-u32">binary-read-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-read-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-binary-read-u64">binary-read-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-read-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-binary-read-uvarint">binary-read-uvarint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-read-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-binary-read-varint">binary-read-varint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-read-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-binary-u16">binary-u16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-u32">binary-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-u64">binary-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-uvarint">binary-uvarint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-varint">binary-varint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-BinaryError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BinaryError</strong></code><span class="doc-sig-links"><a href="#t-BinaryError" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

A binary input or destination cannot satisfy the requested operation.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnexpectedBinaryEnd"><code><strong class="tok-constructor">UnexpectedBinaryEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a></code></li><li id="k-BinaryVarintOverflow"><code><strong class="tok-constructor">BinaryVarintOverflow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ByteOrder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteOrder</strong></code><span class="doc-sig-links"><a href="#t-ByteOrder" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Byte significance for fixed-width wire encodings; independent of host byte order.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-LittleEndian"><code><strong class="tok-constructor">LittleEndian</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a></code></li><li id="k-BigEndian"><code><strong class="tok-constructor">BigEndian</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-binary-put-u16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-put-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-binary-put-u16" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

Write an unsigned 16-bit field into an existing buffer at the zero-based offset. Check all 2 destination bytes before mutation; failure leaves the buffer unchanged.

</div>
</article>

<article class="doc-entry" id="v-binary-put-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-put-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-binary-put-u32" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Write an unsigned 32-bit field into an existing buffer at the zero-based offset. Check all 4 destination bytes before mutation; failure leaves the buffer unchanged.

</div>
</article>

<article class="doc-entry" id="v-binary-put-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-put-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-binary-put-u64" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

Write an unsigned 64-bit field into an existing buffer at the zero-based offset. Check all 8 destination bytes before mutation; failure leaves the buffer unchanged.

</div>
</article>

<article class="doc-entry" id="v-binary-read-u16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-read-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-binary-read-u16" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

Read an unsigned 16-bit field at the zero-based byte offset. Fewer than 2 available bytes returns UnexpectedBinaryEnd; trailing bytes are untouched.

</div>
</article>

<article class="doc-entry" id="v-binary-read-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-read-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-binary-read-u32" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

Read an unsigned 32-bit field at the zero-based byte offset. Fewer than 4 available bytes returns UnexpectedBinaryEnd; trailing bytes are untouched.

</div>
</article>

<article class="doc-entry" id="v-binary-read-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-read-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-binary-read-u64" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

Read an unsigned 64-bit field at the zero-based byte offset. Fewer than 8 available bytes returns UnexpectedBinaryEnd; trailing bytes are untouched.

</div>
</article>

<article class="doc-entry" id="v-binary-read-uvarint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-read-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-binary-read-uvarint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L147" title="View source at line 147">Source</a></span></div>
<div class="doc-body">

Read a terminated unsigned varint at the byte offset and return (value, bytes-consumed). Nonminimal encodings are accepted; incomplete input and 64-bit overflow are distinct errors.

</div>
</article>

<article class="doc-entry" id="v-binary-read-varint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-read-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-BinaryError" title="Text.Binary.BinaryError">BinaryError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-binary-read-varint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

Read and inverse-zigzag an unsigned varint at the byte offset, returning (signed-value, bytes-consumed). Shares the unsigned decoder's truncation and overflow errors.

</div>
</article>

<article class="doc-entry" id="v-binary-u16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-u16" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Allocate exactly 2 bytes encoding an unsigned 16-bit value in the selected byte order.

</div>
</article>

<article class="doc-entry" id="v-binary-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-u32" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

Allocate exactly 4 bytes encoding an unsigned 32-bit value in the selected byte order.

</div>
</article>

<article class="doc-entry" id="v-binary-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-u64" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Allocate exactly 8 bytes encoding an unsigned 64-bit value in the selected byte order.

</div>
</article>

<article class="doc-entry" id="v-binary-uvarint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-uvarint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

Encode a U64 in one to ten little-endian base-128 bytes, with the high bit marking continuation. The emitted representation is minimal.

</div>
</article>

<article class="doc-entry" id="v-binary-varint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-varint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary/source/#L130" title="View source at line 130">Source</a></span></div>
<div class="doc-body">

Zigzag-encode an I64, then encode it as an unsigned varint. Zero maps to 0, -1 to 1, and 1 to 2; both I64 extrema are supported.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Binary.cinch`](/packages/stdlib/Text.Binary/source/). Source SHA-256: `9e8420191ef2f63969bdd1fa54f69724c835a6ada8041b548ee3de8154e71e4d`.
