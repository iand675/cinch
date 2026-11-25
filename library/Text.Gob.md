# Text.Gob

Wire-compatible Go gob binary encoder and decoder for primitives, slices, arrays, maps and structs with stateful type-descriptor reuse.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Gob</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Gob/source/">View source</a><a href="/packages/search/?q=%2BText.Gob">Search module</a><a href="/reference-sources/Text.Gob.cinch" download="Text.Gob.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Wire-compatible gob binary encoder and decoder over typed descriptors and values.
Implements the self-describing gob binary stream format: variable-width big-endian
unsigned integers, zigzag signed integers, byte-reversed binary64 floats, complex128
pairs, length-prefixed byte slices and UTF-8 strings, slices, fixed-length arrays,
maps, and delta-indexed structs with omitted zero fields. Stateful encoders and
decoders retain transmitted user type IDs across multi-message streams so repeated
descriptors are transmitted once. All decoding is bounds-checked and depth-bounded.

</details>

<details class="doc-synopsis"><summary>Synopsis (49 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-GobDecoderState">GobDecoderState</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobDecoderState</strong></code></li><li><a class="syn-name" href="#t-GobEncoderState">GobEncoderState</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobEncoderState</strong></code></li><li><a class="syn-name" href="#t-GobError">GobError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobError</strong></code></li><li><a class="syn-name" href="#t-GobFieldTypes">GobFieldTypes</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobFieldTypes</strong></code></li><li><a class="syn-name" href="#t-GobMapEntries">GobMapEntries</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobMapEntries</strong></code></li><li><a class="syn-name" href="#t-GobStructFields">GobStructFields</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobStructFields</strong></code></li><li><a class="syn-name" href="#t-GobType">GobType</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobType</strong></code></li><li><a class="syn-name" href="#t-GobValue">GobValue</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobValue</strong></code></li><li><a class="syn-name" href="#t-GobValueList">GobValueList</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobValueList</strong></code></li><li><a class="syn-name" href="#v-gob-array">gob-array</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-array</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li><a class="syn-name" href="#v-gob-decode">gob-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>)</code></li><li><a class="syn-name" href="#v-gob-decode-as">gob-decode-as</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-decode-as</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li><a class="syn-name" href="#v-gob-decode-float">gob-decode-float</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-decode-float</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-gob-decode-int">gob-decode-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-decode-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-gob-decode-stream">gob-decode-stream</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-decode-stream</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>))</code></li><li><a class="syn-name" href="#v-gob-decode-uint">gob-decode-uint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-decode-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-gob-decoder-decode-at">gob-decoder-decode-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-decoder-decode-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a>)</code></li><li><a class="syn-name" href="#v-gob-decoder-decode-message">gob-decoder-decode-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-decoder-decode-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>), <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a>)</code></li><li><a class="syn-name" href="#v-gob-decoder-new">gob-decoder-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-decoder-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a></code></li><li><a class="syn-name" href="#v-gob-decoder-type">gob-decoder-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-decoder-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li><a class="syn-name" href="#v-gob-encode">gob-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gob-encode-float">gob-encode-float</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-encode-float</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gob-encode-int">gob-encode-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-encode-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gob-encode-stream">gob-encode-stream</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-encode-stream</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gob-encode-uint">gob-encode-uint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-encode-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gob-encoder-encode">gob-encoder-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-encoder-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobEncoderState" title="Text.Gob.GobEncoderState">GobEncoderState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobEncoderState" title="Text.Gob.GobEncoderState">GobEncoderState</a>)</code></li><li><a class="syn-name" href="#v-gob-encoder-new">gob-encoder-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-encoder-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobEncoderState" title="Text.Gob.GobEncoderState">GobEncoderState</a></code></li><li><a class="syn-name" href="#v-gob-encoder-type-id">gob-encoder-type-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-encoder-type-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobEncoderState" title="Text.Gob.GobEncoderState">GobEncoderState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-gob-error-message">gob-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-gob-field-bool">gob-field-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-field-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-gob-field-bytes">gob-field-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-field-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gob-field-complex">gob-field-complex</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-field-complex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code></li><li><a class="syn-name" href="#v-gob-field-float">gob-field-float</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-field-float</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-gob-field-int">gob-field-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-field-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-gob-field-string">gob-field-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-field-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-gob-field-uint">gob-field-uint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-field-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-gob-is-zero">gob-is-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-is-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-gob-map">gob-map</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-map</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li><a class="syn-name" href="#v-gob-map-entries">gob-map-entries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-map-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>)</code></li><li><a class="syn-name" href="#v-gob-slice">gob-slice</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-slice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li><a class="syn-name" href="#v-gob-slice-items">gob-slice-items</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-slice-items</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li><a class="syn-name" href="#v-gob-struct">gob-struct</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-struct</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li><a class="syn-name" href="#v-gob-struct-field">gob-struct-field</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-struct-field</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li><a class="syn-name" href="#v-gob-struct-fields">gob-struct-fields</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-struct-fields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>)</code></li><li><a class="syn-name" href="#v-gob-type-eq">gob-type-eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-type-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-gob-type-fields">gob-type-fields</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-type-fields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobFieldTypes" title="Text.Gob.GobFieldTypes">GobFieldTypes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>)</code></li><li><a class="syn-name" href="#v-gob-type-struct">gob-type-struct</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-type-struct</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li><a class="syn-name" href="#v-gob-value-eq">gob-value-eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-value-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-gob-zero-value">gob-zero-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-zero-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-GobDecoderState">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobDecoderState</strong></code><span class="doc-sig-links"><a href="#t-GobDecoderState" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L1026" title="View source at line 1026">Source</a></span></div>
<div class="doc-body">

Persistent decoder state mapping transmitted user type IDs to wire descriptors.

</div>
</article>

<article class="doc-entry" id="t-GobEncoderState">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobEncoderState</strong></code><span class="doc-sig-links"><a href="#t-GobEncoderState" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L619" title="View source at line 619">Source</a></span></div>
<div class="doc-body">

Persistent encoder state tracking already-transmitted user types and the next ID.

</div>
</article>

<article class="doc-entry" id="t-GobError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobError</strong></code><span class="doc-sig-links"><a href="#t-GobError" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

Named failures returned when encoding or decoding gob wire streams.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GobTruncated"><code><strong class="tok-constructor">GobTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a></code></li><li id="k-GobOverflow"><code><strong class="tok-constructor">GobOverflow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a></code></li><li id="k-GobUnknownTypeId"><code><strong class="tok-constructor">GobUnknownTypeId</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a></code></li><li id="k-GobMalformedWire"><code><strong class="tok-constructor">GobMalformedWire</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a></code></li><li id="k-GobTypeMismatch"><code><strong class="tok-constructor">GobTypeMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a></code></li><li id="k-GobDepthLimit"><code><strong class="tok-constructor">GobDepthLimit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-GobFieldTypes">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobFieldTypes</strong></code><span class="doc-sig-links"><a href="#t-GobFieldTypes" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

Field descriptors of a gob struct type, kept as a direct recursive spine.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GobFieldTypesEnd"><code><strong class="tok-constructor">GobFieldTypesEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobFieldTypes" title="Text.Gob.GobFieldTypes">GobFieldTypes</a></code></li><li id="k-GobFieldTypeCons"><code><strong class="tok-constructor">GobFieldTypeCons</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobFieldTypes" title="Text.Gob.GobFieldTypes">GobFieldTypes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobFieldTypes" title="Text.Gob.GobFieldTypes">GobFieldTypes</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-GobMapEntries">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobMapEntries</strong></code><span class="doc-sig-links"><a href="#t-GobMapEntries" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Key-value entries of a gob map, kept as a direct recursive spine.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GobMapEnd"><code><strong class="tok-constructor">GobMapEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobMapEntries" title="Text.Gob.GobMapEntries">GobMapEntries</a></code></li><li id="k-GobMapCons"><code><strong class="tok-constructor">GobMapCons</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobMapEntries" title="Text.Gob.GobMapEntries">GobMapEntries</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobMapEntries" title="Text.Gob.GobMapEntries">GobMapEntries</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-GobStructFields">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobStructFields</strong></code><span class="doc-sig-links"><a href="#t-GobStructFields" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L94" title="View source at line 94">Source</a></span></div>
<div class="doc-body">

Named field values of a gob struct, kept as a direct recursive spine.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GobFieldEnd"><code><strong class="tok-constructor">GobFieldEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobStructFields" title="Text.Gob.GobStructFields">GobStructFields</a></code></li><li id="k-GobFieldCons"><code><strong class="tok-constructor">GobFieldCons</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobStructFields" title="Text.Gob.GobStructFields">GobStructFields</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobStructFields" title="Text.Gob.GobStructFields">GobStructFields</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-GobType">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobType</strong></code><span class="doc-sig-links"><a href="#t-GobType" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

A gob wire type descriptor.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GobTBool"><code><strong class="tok-constructor">GobTBool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li id="k-GobTInt"><code><strong class="tok-constructor">GobTInt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li id="k-GobTUint"><code><strong class="tok-constructor">GobTUint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li id="k-GobTFloat"><code><strong class="tok-constructor">GobTFloat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li id="k-GobTBytes"><code><strong class="tok-constructor">GobTBytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li id="k-GobTString"><code><strong class="tok-constructor">GobTString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li id="k-GobTComplex"><code><strong class="tok-constructor">GobTComplex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li id="k-GobTSlice"><code><strong class="tok-constructor">GobTSlice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li id="k-GobTArray"><code><strong class="tok-constructor">GobTArray</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li id="k-GobTMap"><code><strong class="tok-constructor">GobTMap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li id="k-GobTStruct"><code><strong class="tok-constructor">GobTStruct</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobFieldTypes" title="Text.Gob.GobFieldTypes">GobFieldTypes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-GobValue">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobValue</strong></code><span class="doc-sig-links"><a href="#t-GobValue" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

A typed gob value corresponding to a GobType descriptor.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GobBool"><code><strong class="tok-constructor">GobBool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li id="k-GobInt"><code><strong class="tok-constructor">GobInt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li id="k-GobUint"><code><strong class="tok-constructor">GobUint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li id="k-GobFloat"><code><strong class="tok-constructor">GobFloat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li id="k-GobBytes"><code><strong class="tok-constructor">GobBytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li id="k-GobString"><code><strong class="tok-constructor">GobString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li id="k-GobComplex"><code><strong class="tok-constructor">GobComplex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li id="k-GobSlice"><code><strong class="tok-constructor">GobSlice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValueList" title="Text.Gob.GobValueList">GobValueList</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li id="k-GobArray"><code><strong class="tok-constructor">GobArray</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValueList" title="Text.Gob.GobValueList">GobValueList</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li id="k-GobMap"><code><strong class="tok-constructor">GobMap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobMapEntries" title="Text.Gob.GobMapEntries">GobMapEntries</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li id="k-GobStruct"><code><strong class="tok-constructor">GobStruct</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobStructFields" title="Text.Gob.GobStructFields">GobStructFields</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-GobValueList">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobValueList</strong></code><span class="doc-sig-links"><a href="#t-GobValueList" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

Elements of a gob slice or fixed array, kept as a direct recursive spine.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GobValueEnd"><code><strong class="tok-constructor">GobValueEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValueList" title="Text.Gob.GobValueList">GobValueList</a></code></li><li id="k-GobValueCons"><code><strong class="tok-constructor">GobValueCons</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValueList" title="Text.Gob.GobValueList">GobValueList</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValueList" title="Text.Gob.GobValueList">GobValueList</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-gob-array">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-array</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code><span class="doc-sig-links"><a href="#v-gob-array" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L242" title="View source at line 242">Source</a></span></div>
<div class="doc-body">

Construct a gob fixed-length array value from a list of elements.

</div>
</article>

<article class="doc-entry" id="v-gob-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>)</code><span class="doc-sig-links"><a href="#v-gob-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L1588" title="View source at line 1588">Source</a></span></div>
<div class="doc-body">

Decode a single gob value occupying the entire input buffer.

</div>
</article>

<article class="doc-entry" id="v-gob-decode-as">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-decode-as</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code><span class="doc-sig-links"><a href="#v-gob-decode-as" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L1597" title="View source at line 1597">Source</a></span></div>
<div class="doc-body">

Decode a single gob value and verify that its wire type matches the expected GobType.

</div>
</article>

<article class="doc-entry" id="v-gob-decode-float">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-decode-float</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-gob-decode-float" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L543" title="View source at line 543">Source</a></span></div>
<div class="doc-body">

Decode a binary64 float from a gob unsigned integer at byte offset pos.

</div>
</article>

<article class="doc-entry" id="v-gob-decode-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-decode-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-gob-decode-int" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L529" title="View source at line 529">Source</a></span></div>
<div class="doc-body">

Decode a gob signed 64-bit integer at byte offset pos, returning (value, next-offset).

</div>
</article>

<article class="doc-entry" id="v-gob-decode-stream">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-decode-stream</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>))</code><span class="doc-sig-links"><a href="#v-gob-decode-stream" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L1630" title="View source at line 1630">Source</a></span></div>
<div class="doc-body">

Decode every value from a multi-message gob byte buffer.

</div>
</article>

<article class="doc-entry" id="v-gob-decode-uint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-decode-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-gob-decode-uint" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L504" title="View source at line 504">Source</a></span></div>
<div class="doc-body">

Decode a gob unsigned 64-bit integer at byte offset pos, returning (value, next-offset).

</div>
</article>

<article class="doc-entry" id="v-gob-decoder-decode-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-decoder-decode-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a>)</code><span class="doc-sig-links"><a href="#v-gob-decoder-decode-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L1554" title="View source at line 1554">Source</a></span></div>
<div class="doc-body">

Decode one gob value starting at byte offset pos, consuming any preceding type definitions.

</div>
</article>

<article class="doc-entry" id="v-gob-decoder-decode-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-decoder-decode-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>), <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a>)</code><span class="doc-sig-links"><a href="#v-gob-decoder-decode-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L1502" title="View source at line 1502">Source</a></span></div>
<div class="doc-body">

Decode one already-deframed gob message payload (after its unsigned byte length prefix).
Returns None for a type-definition message (updating state) or Some (type, value) for a value message.

</div>
</article>

<article class="doc-entry" id="v-gob-decoder-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-decoder-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a></code><span class="doc-sig-links"><a href="#v-gob-decoder-new" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L1031" title="View source at line 1031">Source</a></span></div>
<div class="doc-body">

Create a fresh gob decoder state with no user type definitions yet received.

</div>
</article>

<article class="doc-entry" id="v-gob-decoder-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-decoder-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobDecoderState" title="Text.Gob.GobDecoderState">GobDecoderState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code><span class="doc-sig-links"><a href="#v-gob-decoder-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L1355" title="View source at line 1355">Source</a></span></div>
<div class="doc-body">

Look up and resolve a wire type ID in a GobDecoderState.

</div>
</article>

<article class="doc-entry" id="v-gob-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gob-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L1581" title="View source at line 1581">Source</a></span></div>
<div class="doc-body">

Encode a single typed value into a complete self-contained gob message buffer.

</div>
</article>

<article class="doc-entry" id="v-gob-encode-float">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-encode-float</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gob-encode-float" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L539" title="View source at line 539">Source</a></span></div>
<div class="doc-body">

Encode a binary64 float using byte-reversed IEEE-754 bits as a gob unsigned integer.

</div>
</article>

<article class="doc-entry" id="v-gob-encode-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-encode-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gob-encode-int" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L520" title="View source at line 520">Source</a></span></div>
<div class="doc-body">

Encode a signed 64-bit integer using gob zigzag encoding.

</div>
</article>

<article class="doc-entry" id="v-gob-encode-stream">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-encode-stream</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gob-encode-stream" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L1616" title="View source at line 1616">Source</a></span></div>
<div class="doc-body">

Encode a sequence of (GobType, GobValue) pairs into one gob stream sharing type IDs.

</div>
</article>

<article class="doc-entry" id="v-gob-encode-uint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-encode-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gob-encode-uint" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L479" title="View source at line 479">Source</a></span></div>
<div class="doc-body">

Encode an unsigned 64-bit integer in gob wire format.
Values 0..127 emit one byte; larger values emit (256 - width) followed by big-endian bytes.

</div>
</article>

<article class="doc-entry" id="v-gob-encoder-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-encoder-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobEncoderState" title="Text.Gob.GobEncoderState">GobEncoderState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobEncoderState" title="Text.Gob.GobEncoderState">GobEncoderState</a>)</code><span class="doc-sig-links"><a href="#v-gob-encoder-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L988" title="View source at line 988">Source</a></span></div>
<div class="doc-body">

Encode one value using persistent encoder state, transmitting any unseen user type descriptors.
Returns the wire bytes for this value (preceded by any new type definitions) and updated state.

</div>
</article>

<article class="doc-entry" id="v-gob-encoder-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-encoder-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobEncoderState" title="Text.Gob.GobEncoderState">GobEncoderState</a></code><span class="doc-sig-links"><a href="#v-gob-encoder-new" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L624" title="View source at line 624">Source</a></span></div>
<div class="doc-body">

Create a fresh gob encoder state with no transmitted user types.

</div>
</article>

<article class="doc-entry" id="v-gob-encoder-type-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-encoder-type-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobEncoderState" title="Text.Gob.GobEncoderState">GobEncoderState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-gob-encoder-type-id" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L635" title="View source at line 635">Source</a></span></div>
<div class="doc-body">

Query the wire type ID of a GobType if it is builtin or already transmitted.

</div>
</article>

<article class="doc-entry" id="v-gob-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-gob-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

Render a human-readable diagnostic message for a GobError.

</div>
</article>

<article class="doc-entry" id="v-gob-field-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-field-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-gob-field-bool" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L287" title="View source at line 287">Source</a></span></div>
<div class="doc-body">

Extract a typed Bool field from a GobStruct value.

</div>
</article>

<article class="doc-entry" id="v-gob-field-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-field-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gob-field-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L319" title="View source at line 319">Source</a></span></div>
<div class="doc-body">

Extract a typed Bytes field from a GobStruct value.

</div>
</article>

<article class="doc-entry" id="v-gob-field-complex">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-field-complex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code><span class="doc-sig-links"><a href="#v-gob-field-complex" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L335" title="View source at line 335">Source</a></span></div>
<div class="doc-body">

Extract a typed complex128 (real, imag) field from a GobStruct value.

</div>
</article>

<article class="doc-entry" id="v-gob-field-float">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-field-float</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-gob-field-float" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L311" title="View source at line 311">Source</a></span></div>
<div class="doc-body">

Extract a typed F64 field from a GobStruct value.

</div>
</article>

<article class="doc-entry" id="v-gob-field-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-field-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-gob-field-int" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L295" title="View source at line 295">Source</a></span></div>
<div class="doc-body">

Extract a typed I64 field from a GobStruct value.

</div>
</article>

<article class="doc-entry" id="v-gob-field-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-field-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-gob-field-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L327" title="View source at line 327">Source</a></span></div>
<div class="doc-body">

Extract a typed String field from a GobStruct value.

</div>
</article>

<article class="doc-entry" id="v-gob-field-uint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-field-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobError" title="Text.Gob.GobError">GobError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-gob-field-uint" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L303" title="View source at line 303">Source</a></span></div>
<div class="doc-body">

Extract a typed U64 field from a GobStruct value.

</div>
</article>

<article class="doc-entry" id="v-gob-is-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-is-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-gob-is-zero" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L420" title="View source at line 420">Source</a></span></div>
<div class="doc-body">

Whether a GobValue is zero for struct field omission rules.
Fixed arrays and nested structs always return False because they are always transmitted.

</div>
</article>

<article class="doc-entry" id="v-gob-map">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-map</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code><span class="doc-sig-links"><a href="#v-gob-map" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L246" title="View source at line 246">Source</a></span></div>
<div class="doc-body">

Construct a gob map value from a list of (key, value) pairs.

</div>
</article>

<article class="doc-entry" id="v-gob-map-entries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-map-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>)</code><span class="doc-sig-links"><a href="#v-gob-map-entries" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L261" title="View source at line 261">Source</a></span></div>
<div class="doc-body">

Extract the key-value entries of a gob map; other values return Nil.

</div>
</article>

<article class="doc-entry" id="v-gob-slice">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-slice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code><span class="doc-sig-links"><a href="#v-gob-slice" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L238" title="View source at line 238">Source</a></span></div>
<div class="doc-body">

Construct a gob slice value from a list of elements.

</div>
</article>

<article class="doc-entry" id="v-gob-slice-items">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-slice-items</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code><span class="doc-sig-links"><a href="#v-gob-slice-items" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L254" title="View source at line 254">Source</a></span></div>
<div class="doc-body">

Extract the elements of a gob slice or fixed array; other values return Nil.

</div>
</article>

<article class="doc-entry" id="v-gob-struct">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-struct</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code><span class="doc-sig-links"><a href="#v-gob-struct" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L250" title="View source at line 250">Source</a></span></div>
<div class="doc-body">

Construct a named gob struct value from a list of (field, value) pairs.

</div>
</article>

<article class="doc-entry" id="v-gob-struct-field">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-struct-field</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code><span class="doc-sig-links"><a href="#v-gob-struct-field" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L281" title="View source at line 281">Source</a></span></div>
<div class="doc-body">

Look up a named field on a GobStruct value.

</div>
</article>

<article class="doc-entry" id="v-gob-struct-fields">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-struct-fields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a>)</code><span class="doc-sig-links"><a href="#v-gob-struct-fields" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L267" title="View source at line 267">Source</a></span></div>
<div class="doc-body">

Extract the named fields of a gob struct; other values return Nil.

</div>
</article>

<article class="doc-entry" id="v-gob-type-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-type-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-gob-type-eq" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

Structural equality between two gob type descriptors.

</div>
</article>

<article class="doc-entry" id="v-gob-type-fields">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-type-fields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobFieldTypes" title="Text.Gob.GobFieldTypes">GobFieldTypes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>)</code><span class="doc-sig-links"><a href="#v-gob-type-fields" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L166" title="View source at line 166">Source</a></span></div>
<div class="doc-body">

Extract the ordered field descriptors of a GobFieldTypes spine as a list.

</div>
</article>

<article class="doc-entry" id="v-gob-type-struct">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-type-struct</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code><span class="doc-sig-links"><a href="#v-gob-type-struct" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L162" title="View source at line 162">Source</a></span></div>
<div class="doc-body">

Construct a named gob struct type from an ordered list of field descriptors.

</div>
</article>

<article class="doc-entry" id="v-gob-value-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-value-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-gob-value-eq" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L368" title="View source at line 368">Source</a></span></div>
<div class="doc-body">

Exact structural equality on GobValue, using bit-exact IEEE-754 comparison for floats.

</div>
</article>

<article class="doc-entry" id="v-gob-zero-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-zero-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code><span class="doc-sig-links"><a href="#v-gob-zero-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Gob/source/#L404" title="View source at line 404">Source</a></span></div>
<div class="doc-body">

Construct the canonical zero value for any GobType descriptor.

</div>
</article>
