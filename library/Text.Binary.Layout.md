# Text.Binary.Layout

Typed scalar, floating-point, complex and aggregate binary layouts with checked buffers, exact sizes and packed array kernels.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Binary.Layout</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Binary.Layout/source/">View source</a><a href="/packages/search/?q=%2BText.Binary.Layout">Search module</a><a href="/reference-sources/Text.Binary.Layout.cinch" download="Text.Binary.Layout.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.Complex](Data.Complex.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

<details class="doc-notes"><summary>Module notes</summary>

Typed fixed-size binary layouts with checked offsets, native wire order and exact float bits.
Layouts describe concrete scalars, arrays, byte fields, padding and products;
pure inverse views adapt products to application record types. Fields have no
implicit alignment. Encoding validates values before allocation and commits
caller-buffer output only after a complete private encoding succeeds.
Decoding returns fresh byte fields and a consumed count, leaving trailing input.

</details>

<details class="doc-synopsis"><summary>Synopsis (55 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-BinaryLayout">BinaryLayout</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BinaryLayout</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-BinaryLayoutError">BinaryLayoutError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BinaryLayoutError</strong></code></li><li><a class="syn-name" href="#t-BinaryOrder">BinaryOrder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BinaryOrder</strong></code></li><li><a class="syn-name" href="#v-binary-append">binary-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-append</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-append-u16">binary-append-u16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-append-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-append-u32">binary-append-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-append-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-append-u64">binary-append-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-append-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-append-uvarint">binary-append-uvarint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-append-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-append-varint">binary-append-varint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-append-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-array">binary-array</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-array</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-binary-bool">binary-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-binary-bytes">binary-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-complex-value">binary-complex-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-complex-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-binary-complex128">binary-complex128</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-complex128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code></li><li><a class="syn-name" href="#v-binary-complex64">binary-complex64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-complex64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>)</code></li><li><a class="syn-name" href="#v-binary-decode">binary-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-decode</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> (<span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-binary-decode-at">binary-decode-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-decode-at</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> (<span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-binary-decode-uvarint">binary-decode-uvarint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-decode-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-binary-decode-varint">binary-decode-varint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-decode-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-binary-encode">binary-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-encode</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-encode-into">binary-encode-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-encode-into</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-binary-encode-uvarint">binary-encode-uvarint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-encode-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-encode-varint">binary-encode-varint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-encode-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-binary-f32">binary-f32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-f32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code></li><li><a class="syn-name" href="#v-binary-f32-bits">binary-f32-bits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-f32-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-binary-f32-from-bits">binary-f32-from-bits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-f32-from-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code></li><li><a class="syn-name" href="#v-binary-f64">binary-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-binary-f64-bits">binary-f64-bits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-f64-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-binary-f64-from-bits">binary-f64-from-bits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-f64-from-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-binary-i16">binary-i16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-i16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I16">I16</a></code></li><li><a class="syn-name" href="#v-binary-i32">binary-i32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-i32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-binary-i64">binary-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-binary-i8">binary-i8</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-i8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I8">I8</a></code></li><li><a class="syn-name" href="#v-binary-layout-error-message">binary-layout-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-layout-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-binary-map">binary-map</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-binary-max-varint16">binary-max-varint16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-max-varint16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-binary-max-varint32">binary-max-varint32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-max-varint32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-binary-max-varint64">binary-max-varint64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-max-varint64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-binary-order-name">binary-order-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-order-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-binary-padding">binary-padding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-binary-pair">binary-pair</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-pair</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)</code></li><li><a class="syn-name" href="#v-binary-put-u16">binary-put-u16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-put-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-binary-put-u32">binary-put-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-put-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-binary-put-u64">binary-put-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-put-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-binary-put-uvarint">binary-put-uvarint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-put-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-binary-put-varint">binary-put-varint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-put-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-binary-read-u16">binary-read-u16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-read-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-binary-read-u32">binary-read-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-read-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-binary-read-u64">binary-read-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-read-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-binary-size">binary-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-binary-u16">binary-u16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-binary-u32">binary-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-binary-u64">binary-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-binary-u8">binary-u8</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-u8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-binary-unit">binary-unit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary-unit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-BinaryLayout">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BinaryLayout</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-BinaryLayout" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

A typed fixed-size wire descriptor; its constructor and trusted buffer callbacks are private.

</div>
</article>

<article class="doc-entry" id="t-BinaryLayoutError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BinaryLayoutError</strong></code><span class="doc-sig-links"><a href="#t-BinaryLayoutError" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Fixed-layout, range and variable-integer failures, with exact progress where available.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-BinaryShortInput"><code><strong class="tok-constructor">BinaryShortInput</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a></code></li><li id="k-BinaryShortDestination"><code><strong class="tok-constructor">BinaryShortDestination</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a></code></li><li id="k-BinaryInvalidOffset"><code><strong class="tok-constructor">BinaryInvalidOffset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a></code></li><li id="k-BinaryLengthMismatch"><code><strong class="tok-constructor">BinaryLengthMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a></code></li><li id="k-BinaryIncompleteVarint"><code><strong class="tok-constructor">BinaryIncompleteVarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a></code></li><li id="k-BinaryVarintOverflow"><code><strong class="tok-constructor">BinaryVarintOverflow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-BinaryOrder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BinaryOrder</strong></code><span class="doc-sig-links"><a href="#t-BinaryOrder" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Wire significance, including an explicit target-native order that retains its own name.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-BinaryLittleEndian"><code><strong class="tok-constructor">BinaryLittleEndian</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a></code></li><li id="k-BinaryBigEndian"><code><strong class="tok-constructor">BinaryBigEndian</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a></code></li><li id="k-BinaryNativeEndian"><code><strong class="tok-constructor">BinaryNativeEndian</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-binary-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-append</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L172" title="View source at line 172">Source</a></span></div>
<div class="doc-body">

Append a complete private encoding to a copied prefix, preserving both caller values.

</div>
</article>

<article class="doc-entry" id="v-binary-append-u16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-append-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-append-u16" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L716" title="View source at line 716">Source</a></span></div>
<div class="doc-body">

Append an unsigned sixteen-bit field without changing the caller's prefix.

</div>
</article>

<article class="doc-entry" id="v-binary-append-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-append-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-append-u32" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L720" title="View source at line 720">Source</a></span></div>
<div class="doc-body">

Append an unsigned thirty-two-bit field without changing the caller's prefix.

</div>
</article>

<article class="doc-entry" id="v-binary-append-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-append-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-append-u64" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L724" title="View source at line 724">Source</a></span></div>
<div class="doc-body">

Append an unsigned sixty-four-bit field without changing the caller's prefix.

</div>
</article>

<article class="doc-entry" id="v-binary-append-uvarint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-append-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-append-uvarint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L764" title="View source at line 764">Source</a></span></div>
<div class="doc-body">

Append a complete unsigned varint to a copied prefix.

</div>
</article>

<article class="doc-entry" id="v-binary-append-varint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-append-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-append-varint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L768" title="View source at line 768">Source</a></span></div>
<div class="doc-body">

Append a complete signed varint to a copied prefix.

</div>
</article>

<article class="doc-entry" id="v-binary-array">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-array</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-binary-array" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L652" title="View source at line 652">Source</a></span></div>
<div class="doc-body">

A fixed-count homogeneous array represented by a list; nested arrays and records are supported.
Encoding rejects length mismatches; decoding obtains its element count from the descriptor.

</div>
</article>

<article class="doc-entry" id="v-binary-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-binary-bool" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L265" title="View source at line 265">Source</a></span></div>
<div class="doc-body">

Encode False as zero and True as one; every nonzero decoded byte is True.

</div>
</article>

<article class="doc-entry" id="v-binary-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L372" title="View source at line 372">Source</a></span></div>
<div class="doc-body">

An exact-size byte field. Decoded values and encoded output never retain a mutable input alias.

</div>
</article>

<article class="doc-entry" id="v-binary-complex-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-complex-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-binary-complex-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L679" title="View source at line 679">Source</a></span></div>
<div class="doc-body">

The standard binary64 complex value, encoded as real then imaginary components in sixteen bytes.

</div>
</article>

<article class="doc-entry" id="v-binary-complex128">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-complex128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code><span class="doc-sig-links"><a href="#v-binary-complex128" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L667" title="View source at line 667">Source</a></span></div>
<div class="doc-body">

A complex binary128 field represented by two binary64 components, real part first.

</div>
</article>

<article class="doc-entry" id="v-binary-complex64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-complex64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>)</code><span class="doc-sig-links"><a href="#v-binary-complex64" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L663" title="View source at line 663">Source</a></span></div>
<div class="doc-body">

A complex binary64 field represented by two binary32 components, real part first.

</div>
</article>

<article class="doc-entry" id="v-binary-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-decode</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> (<span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-binary-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

Decode one complete value and return its consumed byte count; trailing bytes remain available.

</div>
</article>

<article class="doc-entry" id="v-binary-decode-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-decode-at</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> (<span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-binary-decode-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L163" title="View source at line 163">Source</a></span></div>
<div class="doc-body">

Decode at a checked zero-based offset without changing the input or retaining byte-field aliases.

</div>
</article>

<article class="doc-entry" id="v-binary-decode-uvarint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-decode-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-binary-decode-uvarint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L793" title="View source at line 793">Source</a></span></div>
<div class="doc-body">

Decode an unsigned varint with consumed counts, distinguishing incomplete prefixes and overflow.
Complete first-eight-byte quanta reuse the existing SWAR payload compactor.

</div>
</article>

<article class="doc-entry" id="v-binary-decode-varint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-decode-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-binary-decode-varint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L805" title="View source at line 805">Source</a></span></div>
<div class="doc-body">

Decode a signed varint with the same checked prefix counts and full-width inverse zigzag mapping.

</div>
</article>

<article class="doc-entry" id="v-binary-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-encode</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

Encode a validated value into one fresh complete buffer, returning no partial bytes on error.

</div>
</article>

<article class="doc-entry" id="v-binary-encode-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-encode-into</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-binary-encode-into" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L148" title="View source at line 148">Source</a></span></div>
<div class="doc-body">

Encode into an offset only after both complete capacity and value validation succeed.
A private encoding makes nested validation failures and source/destination overlap atomic.

</div>
</article>

<article class="doc-entry" id="v-binary-encode-uvarint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-encode-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-encode-uvarint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L740" title="View source at line 740">Source</a></span></div>
<div class="doc-body">

Encode a complete unsigned integer with its shortest base-128 representation.

</div>
</article>

<article class="doc-entry" id="v-binary-encode-varint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-encode-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary-encode-varint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L744" title="View source at line 744">Source</a></span></div>
<div class="doc-body">

Encode a signed integer by zigzag mapping all I64 values before base-128 encoding.

</div>
</article>

<article class="doc-entry" id="v-binary-f32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-f32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code><span class="doc-sig-links"><a href="#v-binary-f32" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L329" title="View source at line 329">Source</a></span></div>
<div class="doc-body">

One bit-preserving binary32 floating-point field.

</div>
</article>

<article class="doc-entry" id="v-binary-f32-bits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-f32-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-binary-f32-bits" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L301" title="View source at line 301">Source</a></span></div>
<div class="doc-body">

Observe all binary32 bits without arithmetic or NaN canonicalization.

</div>
</article>

<article class="doc-entry" id="v-binary-f32-from-bits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-f32-from-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code><span class="doc-sig-links"><a href="#v-binary-f32-from-bits" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L308" title="View source at line 308">Source</a></span></div>
<div class="doc-body">

Construct binary32 from its complete bit pattern, including signed zeros and NaN payloads.

</div>
</article>

<article class="doc-entry" id="v-binary-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-binary-f64" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L333" title="View source at line 333">Source</a></span></div>
<div class="doc-body">

One bit-preserving binary64 floating-point field.

</div>
</article>

<article class="doc-entry" id="v-binary-f64-bits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-f64-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-binary-f64-bits" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L315" title="View source at line 315">Source</a></span></div>
<div class="doc-body">

Observe all binary64 bits without arithmetic or NaN canonicalization.

</div>
</article>

<article class="doc-entry" id="v-binary-f64-from-bits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-f64-from-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-binary-f64-from-bits" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L322" title="View source at line 322">Source</a></span></div>
<div class="doc-body">

Construct binary64 from its complete bit pattern, including signed zeros and NaN payloads.

</div>
</article>

<article class="doc-entry" id="v-binary-i16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-i16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I16">I16</a></code><span class="doc-sig-links"><a href="#v-binary-i16" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L281" title="View source at line 281">Source</a></span></div>
<div class="doc-body">

One signed two's-complement sixteen-bit field.

</div>
</article>

<article class="doc-entry" id="v-binary-i32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-i32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-binary-i32" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L289" title="View source at line 289">Source</a></span></div>
<div class="doc-body">

One signed two's-complement thirty-two-bit field.

</div>
</article>

<article class="doc-entry" id="v-binary-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-binary-i64" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L297" title="View source at line 297">Source</a></span></div>
<div class="doc-body">

One signed two's-complement sixty-four-bit field.

</div>
</article>

<article class="doc-entry" id="v-binary-i8">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-i8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I8">I8</a></code><span class="doc-sig-links"><a href="#v-binary-i8" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L273" title="View source at line 273">Source</a></span></div>
<div class="doc-body">

One signed two's-complement eight-bit field.

</div>
</article>

<article class="doc-entry" id="v-binary-layout-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-layout-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-binary-layout-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

Render complete required/available counts, invalid offsets or variable-integer progress.

</div>
</article>

<article class="doc-entry" id="v-binary-map">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-binary-map" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L396" title="View source at line 396">Source</a></span></div>
<div class="doc-body">

Adapt a fixed wire view to an application type with pure inverse conversions.
Round trips require the supplied conversions to be inverses on the represented domain.

</div>
</article>

<article class="doc-entry" id="v-binary-max-varint16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-max-varint16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-binary-max-varint16" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L728" title="View source at line 728">Source</a></span></div>
<div class="doc-body">

Maximum bytes for a base-128 integer whose value fits sixteen bits.

</div>
</article>

<article class="doc-entry" id="v-binary-max-varint32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-max-varint32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-binary-max-varint32" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L732" title="View source at line 732">Source</a></span></div>
<div class="doc-body">

Maximum bytes for a base-128 integer whose value fits thirty-two bits.

</div>
</article>

<article class="doc-entry" id="v-binary-max-varint64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-max-varint64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-binary-max-varint64" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L736" title="View source at line 736">Source</a></span></div>
<div class="doc-body">

Maximum bytes for a base-128 integer whose value fits sixty-four bits.

</div>
</article>

<article class="doc-entry" id="v-binary-order-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-order-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-binary-order-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

Stable names for the three wire-order values.

</div>
</article>

<article class="doc-entry" id="v-binary-padding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-binary-padding" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L354" title="View source at line 354">Source</a></span></div>
<div class="doc-body">

Explicit zero-filled padding; decoding skips its bytes without requiring them to be zero.

</div>
</article>

<article class="doc-entry" id="v-binary-pair">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-pair</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)</code><span class="doc-sig-links"><a href="#v-binary-pair" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L382" title="View source at line 382">Source</a></span></div>
<div class="doc-body">

Sequence two checked fields without adding alignment or implicit padding.

</div>
</article>

<article class="doc-entry" id="v-binary-put-u16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-put-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-binary-put-u16" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L695" title="View source at line 695">Source</a></span></div>
<div class="doc-body">

Write an unsigned sixteen-bit field after checking its entire destination interval.

</div>
</article>

<article class="doc-entry" id="v-binary-put-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-put-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-binary-put-u32" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L699" title="View source at line 699">Source</a></span></div>
<div class="doc-body">

Write an unsigned thirty-two-bit field after checking its entire destination interval.

</div>
</article>

<article class="doc-entry" id="v-binary-put-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-put-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-binary-put-u64" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L703" title="View source at line 703">Source</a></span></div>
<div class="doc-body">

Write an unsigned sixty-four-bit field after checking its entire destination interval.

</div>
</article>

<article class="doc-entry" id="v-binary-put-uvarint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-put-uvarint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-binary-put-uvarint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L756" title="View source at line 756">Source</a></span></div>
<div class="doc-body">

Write a complete unsigned varint at an offset; short destinations remain unchanged.

</div>
</article>

<article class="doc-entry" id="v-binary-put-varint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-put-varint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-binary-put-varint" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L760" title="View source at line 760">Source</a></span></div>
<div class="doc-body">

Write a complete signed varint at an offset; short destinations remain unchanged.

</div>
</article>

<article class="doc-entry" id="v-binary-read-u16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-read-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-binary-read-u16" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L683" title="View source at line 683">Source</a></span></div>
<div class="doc-body">

Read an unsigned sixteen-bit field at a checked offset in explicit or native wire order.

</div>
</article>

<article class="doc-entry" id="v-binary-read-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-read-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-binary-read-u32" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L687" title="View source at line 687">Source</a></span></div>
<div class="doc-body">

Read an unsigned thirty-two-bit field at a checked offset.

</div>
</article>

<article class="doc-entry" id="v-binary-read-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-read-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryOrder" title="Text.Binary.Layout.BinaryOrder">BinaryOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayoutError" title="Text.Binary.Layout.BinaryLayoutError">BinaryLayoutError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-binary-read-u64" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L691" title="View source at line 691">Source</a></span></div>
<div class="doc-body">

Read an unsigned sixty-four-bit field at a checked offset.

</div>
</article>

<article class="doc-entry" id="v-binary-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-binary-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

Exact wire bytes described by a layout, independent of its values or target alignment.

</div>
</article>

<article class="doc-entry" id="v-binary-u16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-binary-u16" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L277" title="View source at line 277">Source</a></span></div>
<div class="doc-body">

One unsigned sixteen-bit field.

</div>
</article>

<article class="doc-entry" id="v-binary-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-binary-u32" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L285" title="View source at line 285">Source</a></span></div>
<div class="doc-body">

One unsigned thirty-two-bit field.

</div>
</article>

<article class="doc-entry" id="v-binary-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-binary-u64" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L293" title="View source at line 293">Source</a></span></div>
<div class="doc-body">

One unsigned sixty-four-bit field.

</div>
</article>

<article class="doc-entry" id="v-binary-u8">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-u8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-binary-u8" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L269" title="View source at line 269">Source</a></span></div>
<div class="doc-body">

One unsigned eight-bit field.

</div>
</article>

<article class="doc-entry" id="v-binary-unit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary-unit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary.Layout/#t-BinaryLayout" title="Text.Binary.Layout.BinaryLayout">BinaryLayout</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-binary-unit" title="Permalink">#</a><a href="/packages/stdlib/Text.Binary.Layout/source/#L337" title="View source at line 337">Source</a></span></div>
<div class="doc-body">

Empty fixed layout; encoding emits no bytes and decoding consumes no input.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Binary/Layout.cinch`](/packages/stdlib/Text.Binary.Layout/source/). Source SHA-256: `2bf98a83aaed9aeb72ac3e69d285d15aa28ff65b6543236b659bb775beb8f1d2`.
