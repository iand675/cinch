# Text.Json.Codec

Typed JSON codecs for records, persistent maps, explicit key capabilities, machine integers, binary buffers and exact binary32/binary64 values.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Json.Codec</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Json.Codec/source/">View source</a><a href="/packages/search/?q=%2BText.Json.Codec">Search module</a><a href="/reference-sources/Text.Json.Codec.cinch" download="Text.Json.Codec.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Array](Data.Array.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Json.Binary32](Text.Json.Binary32.md), [Text.Json.Value](Text.Json.Value.md).

<details class="doc-notes"><summary>Module notes</summary>

Typed JSON descriptors, explicit application record capabilities and immutable conversion policies.
Descriptors make type metadata and custom conversion callbacks explicit. Decoding
can retain a partially updated value together with its first structured error.

</details>

<details class="doc-synopsis"><summary>Synopsis (137 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-JsonDecoded">JsonDecoded</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonDecoded</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-JsonEncoded">JsonEncoded</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonEncoded</strong></code></li><li><a class="syn-name" href="#t-JsonField">JsonField</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonField</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-JsonFieldMatch">JsonFieldMatch</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonFieldMatch</strong></code></li><li><a class="syn-name" href="#t-JsonFieldOptions">JsonFieldOptions</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonFieldOptions</strong></code></li><li><a class="syn-name" href="#t-JsonKey">JsonKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonKey</strong> <span class="tok-variable">k</span></code></li><li><a class="syn-name" href="#t-JsonMap">JsonMap</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonMap</strong> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#t-JsonMethods">JsonMethods</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonMethods</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-JsonPolicies">JsonPolicies</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonPolicies</strong></code></li><li><a class="syn-name" href="#t-JsonPolicy">JsonPolicy</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonPolicy</strong></code></li><li><a class="syn-name" href="#t-JsonPolicyOption">JsonPolicyOption</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonPolicyOption</strong></code></li><li><a class="syn-name" href="#t-JsonReadOptions">JsonReadOptions</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonReadOptions</strong></code></li><li><a class="syn-name" href="#t-JsonType">JsonType</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonType</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-JsonWriteOptions">JsonWriteOptions</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonWriteOptions</strong></code></li><li><a class="syn-name" href="#v-json-addressable-methods-only">json-addressable-methods-only</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-addressable-methods-only</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-allow-array-length-mismatch">json-allow-array-length-mismatch</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-allow-array-length-mismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-bool-type">json-bool-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-bool-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-byte-array-numbers">json-byte-array-numbers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-byte-array-numbers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-byte-key">json-byte-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-byte-key</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-variable">k</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span></code></li><li><a class="syn-name" href="#v-json-byte-slice-type">json-byte-slice-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-byte-slice-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-json-bytes-type">json-bytes-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-bytes-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-configure-policies">json-configure-policies</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-configure-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code></li><li><a class="syn-name" href="#v-json-continue-after-type-errors">json-continue-after-type-errors</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-continue-after-type-errors</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-custom-type">json-custom-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-custom-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-decoded-error">json-decoded-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-decoded-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li><a class="syn-name" href="#v-json-decoded-failure">json-decoded-failure</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-decoded-failure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-decoded-fatal">json-decoded-fatal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-decoded-fatal</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-decoded-ok">json-decoded-ok</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-decoded-ok</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-decoded-result">json-decoded-result</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-decoded-result</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-decoded-value">json-decoded-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-decoded-value</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-decoded-with-error">json-decoded-with-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-decoded-with-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-default-field-options">json-default-field-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-default-field-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonFieldOptions" title="Text.Json.Codec.JsonFieldOptions">JsonFieldOptions</a></code></li><li><a class="syn-name" href="#v-json-default-policies">json-default-policies</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-default-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code></li><li><a class="syn-name" href="#v-json-default-read-options">json-default-read-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-default-read-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a></code></li><li><a class="syn-name" href="#v-json-default-write-options">json-default-write-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-default-write-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a></code></li><li><a class="syn-name" href="#v-json-duration-nanoseconds">json-duration-nanoseconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-duration-nanoseconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-empty-policies">json-empty-policies</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-empty-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code></li><li><a class="syn-name" href="#v-json-encoded-bytes">json-encoded-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-encoded-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonEncoded" title="Text.Json.Codec.JsonEncoded">JsonEncoded</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-encoded-error">json-encoded-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-encoded-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonEncoded" title="Text.Json.Codec.JsonEncoded">JsonEncoded</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li><a class="syn-name" href="#v-json-encoded-result">json-encoded-result</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-encoded-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonEncoded" title="Text.Json.Codec.JsonEncoded">JsonEncoded</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-field">json-field</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-field</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonField" title="Text.Json.Codec.JsonField">JsonField</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-field-name">json-field-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-field-name</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonField" title="Text.Json.Codec.JsonField">JsonField</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-field-options">json-field-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-field-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonFieldMatch" title="Text.Json.Codec.JsonFieldMatch">JsonFieldMatch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonFieldOptions" title="Text.Json.Codec.JsonFieldOptions">JsonFieldOptions</a></code></li><li><a class="syn-name" href="#v-json-field-with">json-field-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-field-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonFieldOptions" title="Text.Json.Codec.JsonFieldOptions">JsonFieldOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonField" title="Text.Json.Codec.JsonField">JsonField</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-fixed-array-type">json-fixed-array-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-fixed-array-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-json-fixed-bytes-type">json-fixed-bytes-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-fixed-bytes-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-float32-type">json-float32-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-float32-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code></li><li><a class="syn-name" href="#v-json-float64-type">json-float64-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-float64-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-json-int16-type">json-int16-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-int16-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I16">I16</a></code></li><li><a class="syn-name" href="#v-json-int32-type">json-int32-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-int32-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-json-int64-key">json-int64-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-int64-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-json-int64-type">json-int64-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-int64-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-json-int8-type">json-int8-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-int8-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I8">I8</a></code></li><li><a class="syn-name" href="#v-json-isize-type">json-isize-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-isize-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Isize">Isize</a></code></li><li><a class="syn-name" href="#v-json-json-key">json-json-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-json-key</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span></code></li><li><a class="syn-name" href="#v-json-key">json-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-key</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-variable">k</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span></code></li><li><a class="syn-name" href="#v-json-key-label">json-key-label</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-key-label</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-list-type">json-list-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-list-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-json-loose-base64-padding">json-loose-base64-padding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-loose-base64-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-loose-timestamps">json-loose-timestamps</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-loose-timestamps</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-map-empty">json-map-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-map-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-json-map-entries">json-map-entries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-map-entries</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)</code></li><li><a class="syn-name" href="#v-json-map-from-list">json-map-from-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-map-from-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-json-map-is-null">json-map-is-null</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-map-is-null</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-map-lookup">json-map-lookup</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-map-lookup</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-json-map-null">json-map-null</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-map-null</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-json-map-set">json-map-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-map-set</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-json-map-size">json-map-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-map-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-json-map-type">json-map-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-map-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)</code></li><li><a class="syn-name" href="#v-json-marshal">json-marshal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-marshal</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-marshal-attempt">json-marshal-attempt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-marshal-attempt</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonEncoded" title="Text.Json.Codec.JsonEncoded">JsonEncoded</a></code></li><li><a class="syn-name" href="#v-json-marshal-indent">json-marshal-indent</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-marshal-indent</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-marshal-value">json-marshal-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-marshal-value</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-marshal-with">json-marshal-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-marshal-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-methods">json-methods</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-methods</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMethods" title="Text.Json.Codec.JsonMethods">JsonMethods</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-methods-with-receiver-types">json-methods-with-receiver-types</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-methods-with-receiver-types</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMethods" title="Text.Json.Codec.JsonMethods">JsonMethods</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMethods" title="Text.Json.Codec.JsonMethods">JsonMethods</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-named-bytes-as-binary">json-named-bytes-as-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-named-bytes-as-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-no-methods">json-no-methods</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-no-methods</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMethods" title="Text.Json.Codec.JsonMethods">JsonMethods</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-nullable-type">json-nullable-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-nullable-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-json-number-type">json-number-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code></li><li><a class="syn-name" href="#v-json-omit-empty-application-values">json-omit-empty-application-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-omit-empty-application-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-policies">json-policies</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code></li><li><a class="syn-name" href="#v-json-policy-enabled">json-policy-enabled</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-policy-enabled</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-preserve-name-delimiters">json-preserve-name-delimiters</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-preserve-name-delimiters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-preserve-scalar-null">json-preserve-scalar-null</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-preserve-scalar-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-quote-scalar-fields">json-quote-scalar-fields</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-quote-scalar-fields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code></li><li><a class="syn-name" href="#v-json-raw-type">json-raw-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-raw-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a></code></li><li><a class="syn-name" href="#v-json-read-options">json-read-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-read-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a></code></li><li><a class="syn-name" href="#v-json-read-policies">json-read-policies</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-read-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code></li><li><a class="syn-name" href="#v-json-read-reject-unknown">json-read-reject-unknown</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-read-reject-unknown</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-read-stringify-numbers">json-read-stringify-numbers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-read-stringify-numbers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-read-use-number">json-read-use-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-read-use-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-read-with-stringify-numbers">json-read-with-stringify-numbers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-read-with-stringify-numbers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a></code></li><li><a class="syn-name" href="#v-json-record-type">json-record-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-record-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonField" title="Text.Json.Codec.JsonField">JsonField</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-slice-type">json-slice-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-slice-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>))</code></li><li><a class="syn-name" href="#v-json-string-key">json-string-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-string-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-string-type">json-string-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-string-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-structural-type">json-structural-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-structural-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-type-default">json-type-default</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-default</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-type-is-empty">json-type-is-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-type-is-zero">json-type-is-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-is-zero</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-type-kind">json-type-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-kind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li><a class="syn-name" href="#v-json-type-label">json-type-label</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-label</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-type-read-kind">json-type-read-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-read-kind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li><a class="syn-name" href="#v-json-type-read-name">json-type-read-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-read-name</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-type-with-addressable-encoder">json-type-with-addressable-encoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-with-addressable-encoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-type-with-addressable-text-encoder">json-type-with-addressable-text-encoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-with-addressable-text-encoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-type-with-decoder">json-type-with-decoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-with-decoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-type-with-encoder">json-type-with-encoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-with-encoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-type-with-methods">json-type-with-methods</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-with-methods</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMethods" title="Text.Json.Codec.JsonMethods">JsonMethods</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-type-with-read-identity">json-type-with-read-identity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-with-read-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-type-with-text-decoder">json-type-with-text-decoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-with-text-decoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-type-with-text-encoder">json-type-with-text-encoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-with-text-encoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-type-write-kind">json-type-write-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-write-kind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li><a class="syn-name" href="#v-json-type-write-kind-with">json-type-write-kind-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-write-kind-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li><a class="syn-name" href="#v-json-uint16-type">json-uint16-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-uint16-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-json-uint32-type">json-uint32-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-uint32-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-json-uint64-key">json-uint64-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-uint64-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-json-uint64-type">json-uint64-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-uint64-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-json-uint8-type">json-uint8-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-uint8-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-json-unmarshal">json-unmarshal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-unmarshal</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-unmarshal-with">json-unmarshal-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-unmarshal-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-unsupported-key">json-unsupported-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-unsupported-key</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span></code></li><li><a class="syn-name" href="#v-json-update">json-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-update</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-update-value">json-update-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-update-value</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-update-with">json-update-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-update-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-usize-type">json-usize-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-usize-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a></code></li><li><a class="syn-name" href="#v-json-write-addressable">json-write-addressable</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-addressable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-write-html">json-write-html</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-html</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-write-options">json-write-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a></code></li><li><a class="syn-name" href="#v-json-write-policies">json-write-policies</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code></li><li><a class="syn-name" href="#v-json-write-prefix">json-write-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-write-pretty">json-write-pretty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-pretty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-write-step">json-write-step</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-step</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-write-stringify-numbers">json-write-stringify-numbers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-stringify-numbers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-write-with-addressability">json-write-with-addressability</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-with-addressability</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a></code></li><li><a class="syn-name" href="#v-json-write-with-stringify-numbers">json-write-with-stringify-numbers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-with-stringify-numbers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-JsonDecoded">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonDecoded</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-JsonDecoded" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L258" title="View source at line 258">Source</a></span></div>
<div class="doc-body">

A converted application value, its first error and whether continuation was refused.

</div>
</article>

<article class="doc-entry" id="t-JsonEncoded">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonEncoded</strong></code><span class="doc-sig-links"><a href="#t-JsonEncoded" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L446" title="View source at line 446">Source</a></span></div>
<div class="doc-body">

An encoding outcome retains successful output or the exact prefix emitted before its first refusal.

</div>
</article>

<article class="doc-entry" id="t-JsonField">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonField</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-JsonField" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1320" title="View source at line 1320">Source</a></span></div>
<div class="doc-body">

A field's typed projection/update callbacks are sealed together with its omission metadata.

</div>
</article>

<article class="doc-entry" id="t-JsonFieldMatch">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonFieldMatch</strong></code><span class="doc-sig-links"><a href="#t-JsonFieldMatch" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1298" title="View source at line 1298">Source</a></span></div>
<div class="doc-body">

Per-field name matching takes precedence over record-wide folding behavior.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JsonFieldDefaultMatch"><code><strong class="tok-constructor">JsonFieldDefaultMatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonFieldMatch" title="Text.Json.Codec.JsonFieldMatch">JsonFieldMatch</a></code></li><li id="k-JsonFieldExactMatch"><code><strong class="tok-constructor">JsonFieldExactMatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonFieldMatch" title="Text.Json.Codec.JsonFieldMatch">JsonFieldMatch</a></code></li><li id="k-JsonFieldFoldMatch"><code><strong class="tok-constructor">JsonFieldFoldMatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonFieldMatch" title="Text.Json.Codec.JsonFieldMatch">JsonFieldMatch</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-JsonFieldOptions">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonFieldOptions</strong></code><span class="doc-sig-links"><a href="#t-JsonFieldOptions" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1307" title="View source at line 1307">Source</a></span></div>
<div class="doc-body">

Immutable application field metadata, independent of reflection or source-language tags.

</div>
</article>

<article class="doc-entry" id="t-JsonKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonKey</strong> <span class="tok-variable">k</span></code><span class="doc-sig-links"><a href="#t-JsonKey" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1940" title="View source at line 1940">Source</a></span></div>
<div class="doc-body">

Typed object-key capabilities make ordering and custom text/token methods explicit.

</div>
</article>

<article class="doc-entry" id="t-JsonMap">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonMap</strong> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#t-JsonMap" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2062" title="View source at line 2062">Source</a></span></div>
<div class="doc-body">

Native immutable maps distinguish a null map from an allocated empty map.
Application keys must retain a stable total ordering while stored in a map.

</div>
</article>

<article class="doc-entry" id="t-JsonMethods">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonMethods</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-JsonMethods" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L648" title="View source at line 648">Source</a></span></div>
<div class="doc-body">

Independent JSON/text conversion methods with explicit encoding addressability requirements.

</div>
</article>

<article class="doc-entry" id="t-JsonPolicies">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonPolicies</strong></code><span class="doc-sig-links"><a href="#t-JsonPolicies" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

An immutable set of enabled conversion policies.

</div>
</article>

<article class="doc-entry" id="t-JsonPolicy">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonPolicy</strong></code><span class="doc-sig-links"><a href="#t-JsonPolicy" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

Independently selectable typed conversion semantics.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JsonAddressableMethodsOnly"><code><strong class="tok-constructor">JsonAddressableMethodsOnly</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li><li id="k-JsonByteArrayNumbers"><code><strong class="tok-constructor">JsonByteArrayNumbers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li><li id="k-JsonNamedBytesAsBinary"><code><strong class="tok-constructor">JsonNamedBytesAsBinary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li><li id="k-JsonDurationNanoseconds"><code><strong class="tok-constructor">JsonDurationNanoseconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li><li id="k-JsonPreserveNameDelimiters"><code><strong class="tok-constructor">JsonPreserveNameDelimiters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li><li id="k-JsonPreserveScalarNull"><code><strong class="tok-constructor">JsonPreserveScalarNull</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li><li id="k-JsonOmitEmptyApplicationValues"><code><strong class="tok-constructor">JsonOmitEmptyApplicationValues</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li><li id="k-JsonLooseBase64Padding"><code><strong class="tok-constructor">JsonLooseBase64Padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li><li id="k-JsonLooseTimestamps"><code><strong class="tok-constructor">JsonLooseTimestamps</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li><li id="k-JsonContinueAfterTypeErrors"><code><strong class="tok-constructor">JsonContinueAfterTypeErrors</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li><li id="k-JsonQuoteScalarFields"><code><strong class="tok-constructor">JsonQuoteScalarFields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li><li id="k-JsonAllowArrayLengthMismatch"><code><strong class="tok-constructor">JsonAllowArrayLengthMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-JsonPolicyOption">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonPolicyOption</strong></code><span class="doc-sig-links"><a href="#t-JsonPolicyOption" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

One immutable policy assignment; later assignments override earlier ones.

</div>
</article>

<article class="doc-entry" id="t-JsonReadOptions">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonReadOptions</strong></code><span class="doc-sig-links"><a href="#t-JsonReadOptions" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L225" title="View source at line 225">Source</a></span></div>
<div class="doc-body">

Decoding configuration independent of any particular application type.

</div>
</article>

<article class="doc-entry" id="t-JsonType">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonType</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-JsonType" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L291" title="View source at line 291">Source</a></span></div>
<div class="doc-body">

Explicit type metadata and conversion callbacks replace implicit runtime reflection.

</div>
</article>

<article class="doc-entry" id="t-JsonWriteOptions">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonWriteOptions</strong></code><span class="doc-sig-links"><a href="#t-JsonWriteOptions" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L175" title="View source at line 175">Source</a></span></div>
<div class="doc-body">

Encoding configuration shared by nested descriptors and the final formatter.

</div>
</article>

# Values

<article class="doc-entry" id="v-json-addressable-methods-only">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-addressable-methods-only</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-addressable-methods-only" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

Addressability-sensitive custom methods and text-first object keys.

</div>
</article>

<article class="doc-entry" id="v-json-allow-array-length-mismatch">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-allow-array-length-mismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-allow-array-length-mismatch" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L171" title="View source at line 171">Source</a></span></div>
<div class="doc-body">

Fixed arrays accept different source lengths, zeroing unused elements.

</div>
</article>

<article class="doc-entry" id="v-json-bool-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-bool-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-bool-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L792" title="View source at line 792">Source</a></span></div>
<div class="doc-body">

The boolean descriptor with false as both zero and empty.

</div>
</article>

<article class="doc-entry" id="v-json-byte-array-numbers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-byte-array-numbers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-byte-array-numbers" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

Fixed byte arrays use numeric arrays instead of binary text.

</div>
</article>

<article class="doc-entry" id="v-json-byte-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-byte-key</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-variable">k</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span></code><span class="doc-sig-links"><a href="#v-json-byte-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1959" title="View source at line 1959">Source</a></span></div>
<div class="doc-body">

Convert raw member-name bytes, normalizing malformed UTF-8 before sorting and JSON quoting.

</div>
</article>

<article class="doc-entry" id="v-json-byte-slice-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-byte-slice-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-json-byte-slice-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1890" title="View source at line 1890">Source</a></span></div>
<div class="doc-body">

Nullable binary byte slices preserve the null/empty distinction and numeric-array update behavior.

</div>
</article>

<article class="doc-entry" id="v-json-bytes-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-bytes-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-bytes-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1858" title="View source at line 1858">Source</a></span></div>
<div class="doc-body">

Non-null binary bytes; null clears to an empty buffer and marshaling always emits binary text.

</div>
</article>

<article class="doc-entry" id="v-json-configure-policies">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-configure-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code><span class="doc-sig-links"><a href="#v-json-configure-policies" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

Apply assignments left to right, retaining the last setting for each policy.

</div>
</article>

<article class="doc-entry" id="v-json-continue-after-type-errors">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-continue-after-type-errors</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-continue-after-type-errors" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L163" title="View source at line 163">Source</a></span></div>
<div class="doc-body">

Recoverable field mismatches retain the first error and continue.

</div>
</article>

<article class="doc-entry" id="v-json-custom-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-custom-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-custom-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L575" title="View source at line 575">Source</a></span></div>
<div class="doc-body">

Construct a custom typed capability with explicit defaults, omission tests and update outcomes.

</div>
</article>

<article class="doc-entry" id="v-json-decoded-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-decoded-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code><span class="doc-sig-links"><a href="#v-json-decoded-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L279" title="View source at line 279">Source</a></span></div>
<div class="doc-body">

Inspect the first retained structured conversion error.

</div>
</article>

<article class="doc-entry" id="v-json-decoded-failure">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-decoded-failure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-decoded-failure" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L267" title="View source at line 267">Source</a></span></div>
<div class="doc-body">

Construct an unrecoverable custom conversion refusal while retaining the supplied value.

</div>
</article>

<article class="doc-entry" id="v-json-decoded-fatal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-decoded-fatal</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-decoded-fatal" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L283" title="View source at line 283">Source</a></span></div>
<div class="doc-body">

Whether a custom callback or syntax failure stopped subsequent updates.

</div>
</article>

<article class="doc-entry" id="v-json-decoded-ok">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-decoded-ok</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-decoded-ok" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L263" title="View source at line 263">Source</a></span></div>
<div class="doc-body">

Construct a successful descriptor result.

</div>
</article>

<article class="doc-entry" id="v-json-decoded-result">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-decoded-result</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-decoded-result" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L287" title="View source at line 287">Source</a></span></div>
<div class="doc-body">

Require successful conversion when a partial application value is not useful.

</div>
</article>

<article class="doc-entry" id="v-json-decoded-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-decoded-value</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-decoded-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L275" title="View source at line 275">Source</a></span></div>
<div class="doc-body">

Read the resulting application value even when conversion also reported an error.

</div>
</article>

<article class="doc-entry" id="v-json-decoded-with-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-decoded-with-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-decoded-with-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L271" title="View source at line 271">Source</a></span></div>
<div class="doc-body">

Construct a recoverable type/range mismatch while retaining the supplied value.

</div>
</article>

<article class="doc-entry" id="v-json-default-field-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-default-field-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonFieldOptions" title="Text.Json.Codec.JsonFieldOptions">JsonFieldOptions</a></code><span class="doc-sig-links"><a href="#v-json-default-field-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1316" title="View source at line 1316">Source</a></span></div>
<div class="doc-body">

Ordinary visible fields participate at depth zero without omission or quoting.

</div>
</article>

<article class="doc-entry" id="v-json-default-policies">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-default-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code><span class="doc-sig-links"><a href="#v-json-default-policies" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Established permissive, merge-preserving typed conversion semantics.

</div>
</article>

<article class="doc-entry" id="v-json-default-read-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-default-read-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a></code><span class="doc-sig-links"><a href="#v-json-default-read-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L234" title="View source at line 234">Source</a></span></div>
<div class="doc-body">

Floating dynamic numbers, ignored unknown fields and established merge semantics.

</div>
</article>

<article class="doc-entry" id="v-json-default-write-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-default-write-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a></code><span class="doc-sig-links"><a href="#v-json-default-write-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L184" title="View source at line 184">Source</a></span></div>
<div class="doc-body">

Compact encoding with HTML escaping and established typed semantics.

</div>
</article>

<article class="doc-entry" id="v-json-duration-nanoseconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-duration-nanoseconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-duration-nanoseconds" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L139" title="View source at line 139">Source</a></span></div>
<div class="doc-body">

Duration descriptors encode an integral nanosecond count.

</div>
</article>

<article class="doc-entry" id="v-json-empty-policies">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-empty-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code><span class="doc-sig-links"><a href="#v-json-empty-policies" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

Explicitly disable all conversion choices before applying selected assignments.

</div>
</article>

<article class="doc-entry" id="v-json-encoded-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-encoded-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonEncoded" title="Text.Json.Codec.JsonEncoded">JsonEncoded</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-encoded-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L451" title="View source at line 451">Source</a></span></div>
<div class="doc-body">

Export an independent copy of successful output or the retained partial prefix.

</div>
</article>

<article class="doc-entry" id="v-json-encoded-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-encoded-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonEncoded" title="Text.Json.Codec.JsonEncoded">JsonEncoded</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code><span class="doc-sig-links"><a href="#v-json-encoded-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L455" title="View source at line 455">Source</a></span></div>
<div class="doc-body">

Inspect the original diagnostic without exposing internal output-transport wrappers.

</div>
</article>

<article class="doc-entry" id="v-json-encoded-result">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-encoded-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonEncoded" title="Text.Json.Codec.JsonEncoded">JsonEncoded</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-encoded-result" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L459" title="View source at line 459">Source</a></span></div>
<div class="doc-body">

Require successful encoding when a partial output prefix is not useful.

</div>
</article>

<article class="doc-entry" id="v-json-field">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-field</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonField" title="Text.Json.Codec.JsonField">JsonField</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-field" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1415" title="View source at line 1415">Source</a></span></div>
<div class="doc-body">

Construct an ordinary visible field without omission, quoting or embedding metadata.

</div>
</article>

<article class="doc-entry" id="v-json-field-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-field-name</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonField" title="Text.Json.Codec.JsonField">JsonField</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-field-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1325" title="View source at line 1325">Source</a></span></div>
<div class="doc-body">

Inspect a field's declared JSON member name.

</div>
</article>

<article class="doc-entry" id="v-json-field-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-field-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonFieldMatch" title="Text.Json.Codec.JsonFieldMatch">JsonFieldMatch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonFieldOptions" title="Text.Json.Codec.JsonFieldOptions">JsonFieldOptions</a></code><span class="doc-sig-links"><a href="#v-json-field-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1312" title="View source at line 1312">Source</a></span></div>
<div class="doc-body">

Configure explicit omission, quoting, embedding dominance and member-name matching.

</div>
</article>

<article class="doc-entry" id="v-json-field-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-field-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonFieldOptions" title="Text.Json.Codec.JsonFieldOptions">JsonFieldOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonField" title="Text.Json.Codec.JsonField">JsonField</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-field-with" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1406" title="View source at line 1406">Source</a></span></div>
<div class="doc-body">

Construct one typed field using explicit projection and functional replacement capabilities.

</div>
</article>

<article class="doc-entry" id="v-json-fixed-array-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-fixed-array-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-json-fixed-array-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1091" title="View source at line 1091">Source</a></span></div>
<div class="doc-body">

Compose an exact fixed-array descriptor with immutable element defaults and checked lengths.

</div>
</article>

<article class="doc-entry" id="v-json-fixed-bytes-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-fixed-bytes-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-fixed-bytes-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1927" title="View source at line 1927">Source</a></span></div>
<div class="doc-body">

Fixed-size owned byte buffers, numeric by default, with explicit binary-string alternate policy.

</div>
</article>

<article class="doc-entry" id="v-json-float32-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-float32-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code><span class="doc-sig-links"><a href="#v-json-float32-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1215" title="View source at line 1215">Source</a></span></div>
<div class="doc-body">

The finite binary32 descriptor with exact rounding and shortest JSON decimal formatting.

</div>
</article>

<article class="doc-entry" id="v-json-float64-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-float64-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-json-float64-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1191" title="View source at line 1191">Source</a></span></div>
<div class="doc-body">

The finite binary64 descriptor with shortest JSON decimal formatting.

</div>
</article>

<article class="doc-entry" id="v-json-int16-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-int16-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I16">I16</a></code><span class="doc-sig-links"><a href="#v-json-int16-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1733" title="View source at line 1733">Source</a></span></div>
<div class="doc-body">

Signed sixteen-bit integers with exact range refusal.

</div>
</article>

<article class="doc-entry" id="v-json-int32-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-int32-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-json-int32-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1736" title="View source at line 1736">Source</a></span></div>
<div class="doc-body">

Signed thirty-two-bit integers with exact range refusal.

</div>
</article>

<article class="doc-entry" id="v-json-int64-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-int64-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-json-int64-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2132" title="View source at line 2132">Source</a></span></div>
<div class="doc-body">

Signed64 keys use exact decimal text and original member-name range diagnostics.

</div>
</article>

<article class="doc-entry" id="v-json-int64-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-int64-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-json-int64-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L843" title="View source at line 843">Source</a></span></div>
<div class="doc-body">

The exact signed 64-bit integer descriptor.

</div>
</article>

<article class="doc-entry" id="v-json-int8-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-int8-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I8">I8</a></code><span class="doc-sig-links"><a href="#v-json-int8-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1730" title="View source at line 1730">Source</a></span></div>
<div class="doc-body">

Signed eight-bit integers with exact range refusal.

</div>
</article>

<article class="doc-entry" id="v-json-isize-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-isize-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Isize">Isize</a></code><span class="doc-sig-links"><a href="#v-json-isize-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1739" title="View source at line 1739">Source</a></span></div>
<div class="doc-body">

Target-width signed integers with platform-specific checked bounds.

</div>
</article>

<article class="doc-entry" id="v-json-json-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-json-key</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span></code><span class="doc-sig-links"><a href="#v-json-json-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2045" title="View source at line 2045">Source</a></span></div>
<div class="doc-body">

Use typed JSON key capabilities with numeric stringification and explicit invalid-member-name errors.
Dedicated string/integer key descriptors provide their ordinary primitive text rules.

</div>
</article>

<article class="doc-entry" id="v-json-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-key</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-variable">k</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span></code><span class="doc-sig-links"><a href="#v-json-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1947" title="View source at line 1947">Source</a></span></div>
<div class="doc-body">

Construct an object-key descriptor with explicit comparator and conversion capabilities.

</div>
</article>

<article class="doc-entry" id="v-json-key-label">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-key-label</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-key-label" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2053" title="View source at line 2053">Source</a></span></div>
<div class="doc-body">

Observe the declared application key family.

</div>
</article>

<article class="doc-entry" id="v-json-list-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-list-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-json-list-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1025" title="View source at line 1025">Source</a></span></div>
<div class="doc-body">

Compose non-null arrays represented by ordinary immutable lists.

</div>
</article>

<article class="doc-entry" id="v-json-loose-base64-padding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-loose-base64-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-loose-base64-padding" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L155" title="View source at line 155">Source</a></span></div>
<div class="doc-body">

Binary text permits embedded CR/LF while preserving the padded alphabet grammar.

</div>
</article>

<article class="doc-entry" id="v-json-loose-timestamps">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-loose-timestamps</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-loose-timestamps" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

Time descriptors permit the explicitly documented compatibility spellings.

</div>
</article>

<article class="doc-entry" id="v-json-map-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-map-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-json-map-empty" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2071" title="View source at line 2071">Source</a></span></div>
<div class="doc-body">

Construct an allocated empty map, which marshals as an empty object.

</div>
</article>

<article class="doc-entry" id="v-json-map-entries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-map-entries</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)</code><span class="doc-sig-links"><a href="#v-json-map-entries" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2091" title="View source at line 2091">Source</a></span></div>
<div class="doc-body">

Enumerate application keys in their comparator's order, independently of encoded JSON name sorting.

</div>
</article>

<article class="doc-entry" id="v-json-map-from-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-map-from-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-json-map-from-list" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2099" title="View source at line 2099">Source</a></span></div>
<div class="doc-body">

Construct an allocated map from ordered entries; later comparator-equal keys replace earlier ones.

</div>
</article>

<article class="doc-entry" id="v-json-map-is-null">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-map-is-null</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-map-is-null" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2075" title="View source at line 2075">Source</a></span></div>
<div class="doc-body">

Observe nullness independently of the number of stored members.

</div>
</article>

<article class="doc-entry" id="v-json-map-lookup">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-map-lookup</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-json-map-lookup" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2083" title="View source at line 2083">Source</a></span></div>
<div class="doc-body">

Look up one application key along a logarithmically bounded tail traversal.

</div>
</article>

<article class="doc-entry" id="v-json-map-null">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-map-null</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-json-map-null" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2067" title="View source at line 2067">Source</a></span></div>
<div class="doc-body">

Construct a null map retaining the supplied key family's ordering capability.

</div>
</article>

<article class="doc-entry" id="v-json-map-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-map-set</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-json-map-set" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2087" title="View source at line 2087">Source</a></span></div>
<div class="doc-body">

Persistently insert or replace an application key, allocating a null map on its first insertion.

</div>
</article>

<article class="doc-entry" id="v-json-map-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-map-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-json-map-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2079" title="View source at line 2079">Source</a></span></div>
<div class="doc-body">

Read the cached map size in constant time.

</div>
</article>

<article class="doc-entry" id="v-json-map-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-map-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMap" title="Text.Json.Codec.JsonMap">JsonMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)</code><span class="doc-sig-links"><a href="#v-json-map-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2237" title="View source at line 2237">Source</a></span></div>
<div class="doc-body">

Compose a map descriptor with explicit key capabilities, sorted encoding and logarithmic updates.
The same application ordering must be used to construct prior map values.

</div>
</article>

<article class="doc-entry" id="v-json-marshal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-marshal</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-marshal" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L479" title="View source at line 479">Source</a></span></div>
<div class="doc-body">

Marshal one complete compact value with established defaults.

</div>
</article>

<article class="doc-entry" id="v-json-marshal-attempt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-marshal-attempt</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonEncoded" title="Text.Json.Codec.JsonEncoded">JsonEncoded</a></code><span class="doc-sig-links"><a href="#v-json-marshal-attempt" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L463" title="View source at line 463">Source</a></span></div>
<div class="doc-body">

Marshal with policy-selected partial output and the original error family on failure.

</div>
</article>

<article class="doc-entry" id="v-json-marshal-indent">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-marshal-indent</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-marshal-indent" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L483" title="View source at line 483">Source</a></span></div>
<div class="doc-body">

Marshal one value with an explicit indentation pass, including empty prefix and step strings.

</div>
</article>

<article class="doc-entry" id="v-json-marshal-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-marshal-value</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-marshal-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L362" title="View source at line 362">Source</a></span></div>
<div class="doc-body">

Encode one compact fragment with a directly inspectable original error family.
Complete-fragment callbacks return their own refusal without exposing the
private partial-output transport used between ordinary containers.

</div>
</article>

<article class="doc-entry" id="v-json-marshal-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-marshal-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-marshal-with" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L473" title="View source at line 473">Source</a></span></div>
<div class="doc-body">

Marshal one complete value using explicit HTML, indentation and conversion policies.

</div>
</article>

<article class="doc-entry" id="v-json-methods">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-methods</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMethods" title="Text.Json.Codec.JsonMethods">JsonMethods</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-methods" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L653" title="View source at line 653">Source</a></span></div>
<div class="doc-body">

Compose optional methods; JSON takes precedence over text in each direction when its encoding receiver is eligible.

</div>
</article>

<article class="doc-entry" id="v-json-methods-with-receiver-types">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-methods-with-receiver-types</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMethods" title="Text.Json.Codec.JsonMethods">JsonMethods</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMethods" title="Text.Json.Codec.JsonMethods">JsonMethods</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-methods-with-receiver-types" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L661" title="View source at line 661">Source</a></span></div>
<div class="doc-body">

Set the independently named JSON and text encoding receivers without changing the application value type.

</div>
</article>

<article class="doc-entry" id="v-json-named-bytes-as-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-named-bytes-as-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-named-bytes-as-binary" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

Binary text formatting permits named byte-element descriptors.

</div>
</article>

<article class="doc-entry" id="v-json-no-methods">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-no-methods</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMethods" title="Text.Json.Codec.JsonMethods">JsonMethods</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-no-methods" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L657" title="View source at line 657">Source</a></span></div>
<div class="doc-body">

An empty method set preserves every ordinary descriptor capability.

</div>
</article>

<article class="doc-entry" id="v-json-nullable-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-nullable-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-json-nullable-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L918" title="View source at line 918">Source</a></span></div>
<div class="doc-body">

Compose an explicit nullable/pointee descriptor with ordinary typed update semantics.

</div>
</article>

<article class="doc-entry" id="v-json-number-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code><span class="doc-sig-links"><a href="#v-json-number-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L876" title="View source at line 876">Source</a></span></div>
<div class="doc-body">

Exact number lexemes with checked conversions and an absent-spelling zero default.

</div>
</article>

<article class="doc-entry" id="v-json-omit-empty-application-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-omit-empty-application-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-omit-empty-application-values" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L151" title="View source at line 151">Source</a></span></div>
<div class="doc-body">

Empty-field omission uses the application value classification.

</div>
</article>

<article class="doc-entry" id="v-json-policies">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code><span class="doc-sig-links"><a href="#v-json-policies" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

Configure the established defaults using explicit immutable assignments.

</div>
</article>

<article class="doc-entry" id="v-json-policy-enabled">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-policy-enabled</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicy" title="Text.Json.Codec.JsonPolicy">JsonPolicy</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-policy-enabled" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

Inspect one named policy without revealing its representation.

</div>
</article>

<article class="doc-entry" id="v-json-preserve-name-delimiters">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-preserve-name-delimiters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-preserve-name-delimiters" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

Case-insensitive record matching preserves underscores and dashes.

</div>
</article>

<article class="doc-entry" id="v-json-preserve-scalar-null">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-preserve-scalar-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-preserve-scalar-null" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L147" title="View source at line 147">Source</a></span></div>
<div class="doc-body">

Null scalars retain their value and sequence elements merge.

</div>
</article>

<article class="doc-entry" id="v-json-quote-scalar-fields">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-quote-scalar-fields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicyOption" title="Text.Json.Codec.JsonPolicyOption">JsonPolicyOption</a></code><span class="doc-sig-links"><a href="#v-json-quote-scalar-fields" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Quoted fields permit boolean and string values as well as numbers.

</div>
</article>

<article class="doc-entry" id="v-json-raw-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-raw-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a></code><span class="doc-sig-links"><a href="#v-json-raw-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L898" title="View source at line 898">Source</a></span></div>
<div class="doc-body">

Owned raw JSON fragments with copying assignment and validated marshaling.

</div>
</article>

<article class="doc-entry" id="v-json-read-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-read-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a></code><span class="doc-sig-links"><a href="#v-json-read-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L230" title="View source at line 230">Source</a></span></div>
<div class="doc-body">

Construct immutable decoding options without mutating an existing decoder branch.

</div>
</article>

<article class="doc-entry" id="v-json-read-policies">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-read-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code><span class="doc-sig-links"><a href="#v-json-read-policies" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L246" title="View source at line 246">Source</a></span></div>
<div class="doc-body">

Typed conversion choices used while applying a decoded value.

</div>
</article>

<article class="doc-entry" id="v-json-read-reject-unknown">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-read-reject-unknown</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-read-reject-unknown" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L242" title="View source at line 242">Source</a></span></div>
<div class="doc-body">

Whether a member unmatched by a record descriptor is an error.

</div>
</article>

<article class="doc-entry" id="v-json-read-stringify-numbers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-read-stringify-numbers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-read-stringify-numbers" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L250" title="View source at line 250">Source</a></span></div>
<div class="doc-body">

Whether numeric descriptors require quoted source numbers in this nested context.

</div>
</article>

<article class="doc-entry" id="v-json-read-use-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-read-use-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-read-use-number" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L238" title="View source at line 238">Source</a></span></div>
<div class="doc-body">

Whether dynamic-value decoding preserves exact numbers instead of converting to floats.

</div>
</article>

<article class="doc-entry" id="v-json-read-with-stringify-numbers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-read-with-stringify-numbers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a></code><span class="doc-sig-links"><a href="#v-json-read-with-stringify-numbers" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L254" title="View source at line 254">Source</a></span></div>
<div class="doc-body">

Select nested numeric quoting without changing dynamic-number or unknown-field policies.

</div>
</article>

<article class="doc-entry" id="v-json-record-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-record-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonField" title="Text.Json.Codec.JsonField">JsonField</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-record-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1618" title="View source at line 1618">Source</a></span></div>
<div class="doc-body">

Construct a record descriptor with explicit default/zero capabilities and compiled field dominance.

</div>
</article>

<article class="doc-entry" id="v-json-slice-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-slice-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>))</code><span class="doc-sig-links"><a href="#v-json-slice-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1029" title="View source at line 1029">Source</a></span></div>
<div class="doc-body">

Distinguish a null sequence from a present empty array using an explicit optional list.

</div>
</article>

<article class="doc-entry" id="v-json-string-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-string-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-string-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2103" title="View source at line 2103">Source</a></span></div>
<div class="doc-body">

String keys preserve exact decoded spelling and sort by their encoded UTF-8 bytes.

</div>
</article>

<article class="doc-entry" id="v-json-string-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-string-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-string-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L806" title="View source at line 806">Source</a></span></div>
<div class="doc-body">

The scalar-string descriptor with empty text as both zero and empty.

</div>
</article>

<article class="doc-entry" id="v-json-structural-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-structural-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-structural-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L589" title="View source at line 589">Source</a></span></div>
<div class="doc-body">

Compose a structural descriptor with an explicit category and validated output; child errors retain their original identity.
The category controls containing-field quoting. Successful callback fragments
are checked as complete JSON values before entering a containing descriptor.

</div>
</article>

<article class="doc-entry" id="v-json-type-default">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-default</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-type-default" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L340" title="View source at line 340">Source</a></span></div>
<div class="doc-body">

Construct a fresh default application value through its descriptor capability.

</div>
</article>

<article class="doc-entry" id="v-json-type-is-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-type-is-empty" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L348" title="View source at line 348">Source</a></span></div>
<div class="doc-body">

Apply the descriptor's application empty classifier.

</div>
</article>

<article class="doc-entry" id="v-json-type-is-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-is-zero</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-type-is-zero" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L344" title="View source at line 344">Source</a></span></div>
<div class="doc-body">

Apply the descriptor's application zero classifier.

</div>
</article>

<article class="doc-entry" id="v-json-type-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-kind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code><span class="doc-sig-links"><a href="#v-json-type-kind" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L296" title="View source at line 296">Source</a></span></div>
<div class="doc-body">

Established write representation category; custom/raw fragments use the explicit unspecified category.

</div>
</article>

<article class="doc-entry" id="v-json-type-label">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-label</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-type-label" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L312" title="View source at line 312">Source</a></span></div>
<div class="doc-body">

Application type name attached to structured conversion failures.

</div>
</article>

<article class="doc-entry" id="v-json-type-read-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-read-kind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code><span class="doc-sig-links"><a href="#v-json-type-read-kind" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L308" title="View source at line 308">Source</a></span></div>
<div class="doc-body">

Inspect the decoding category independently of encoding methods and their field-quoting behavior.

</div>
</article>

<article class="doc-entry" id="v-json-type-read-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-read-name</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-type-read-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L316" title="View source at line 316">Source</a></span></div>
<div class="doc-body">

Inspect the independent root name used when reporting an application field path.

</div>
</article>

<article class="doc-entry" id="v-json-type-with-addressable-encoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-with-addressable-encoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-type-with-addressable-encoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L611" title="View source at line 611">Source</a></span></div>
<div class="doc-body">

Add an addressable-only JSON encoder with ordinary fallback for nonaddressable application values.

</div>
</article>

<article class="doc-entry" id="v-json-type-with-addressable-text-encoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-with-addressable-text-encoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-type-with-addressable-text-encoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L615" title="View source at line 615">Source</a></span></div>
<div class="doc-body">

Add an addressable-only text encoder with ordinary fallback and the current HTML escaping policy.

</div>
</article>

<article class="doc-entry" id="v-json-type-with-decoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-with-decoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-type-with-decoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L619" title="View source at line 619">Source</a></span></div>
<div class="doc-body">

Replace only JSON decoding, preserving ordinary encoding and its containing-field quoting behavior.

</div>
</article>

<article class="doc-entry" id="v-json-type-with-encoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-with-encoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-type-with-encoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L595" title="View source at line 595">Source</a></span></div>
<div class="doc-body">

Replace only JSON encoding, preserving ordinary decoding, defaults and application omission predicates.
Successful method fragments are validated and snapshotted before joining a
containing value. A method controls its own encoding representation.

</div>
</article>

<article class="doc-entry" id="v-json-type-with-methods">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-with-methods</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonMethods" title="Text.Json.Codec.JsonMethods">JsonMethods</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-type-with-methods" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L777" title="View source at line 777">Source</a></span></div>
<div class="doc-body">

Install independent methods while preserving defaults, omission predicates and ordinary fallback capabilities.

</div>
</article>

<article class="doc-entry" id="v-json-type-with-read-identity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-with-read-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-type-with-read-identity" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L335" title="View source at line 335">Source</a></span></div>
<div class="doc-body">

Supply independent application and root reporting names without inferring identities from display labels.

</div>
</article>

<article class="doc-entry" id="v-json-type-with-text-decoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-with-text-decoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-type-with-text-decoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L643" title="View source at line 643">Source</a></span></div>
<div class="doc-body">

Replace only text decoding, preserving encoding and passing independently owned decoded string bytes.

</div>
</article>

<article class="doc-entry" id="v-json-type-with-text-encoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-with-text-encoder</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-type-with-text-encoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L630" title="View source at line 630">Source</a></span></div>
<div class="doc-body">

Replace only text encoding; method output is a JSON string and ordinary decoding retains its original kind.

</div>
</article>

<article class="doc-entry" id="v-json-type-write-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-write-kind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code><span class="doc-sig-links"><a href="#v-json-type-write-kind" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L300" title="View source at line 300">Source</a></span></div>
<div class="doc-body">

Inspect the encoding category independently of decoding methods and their field-quoting behavior.

</div>
</article>

<article class="doc-entry" id="v-json-type-write-kind-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-write-kind-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code><span class="doc-sig-links"><a href="#v-json-type-write-kind-with" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L304" title="View source at line 304">Source</a></span></div>
<div class="doc-body">

Inspect the encoding category in the exact context used to select an encoding method.

</div>
</article>

<article class="doc-entry" id="v-json-uint16-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-uint16-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-json-uint16-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1745" title="View source at line 1745">Source</a></span></div>
<div class="doc-body">

Unsigned sixteen-bit integers with exact range refusal.

</div>
</article>

<article class="doc-entry" id="v-json-uint32-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-uint32-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-json-uint32-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1748" title="View source at line 1748">Source</a></span></div>
<div class="doc-body">

Unsigned thirty-two-bit integers with exact range refusal.

</div>
</article>

<article class="doc-entry" id="v-json-uint64-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-uint64-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-json-uint64-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2138" title="View source at line 2138">Source</a></span></div>
<div class="doc-body">

Unsigned64 keys use full-range decimal text and eight-digit SWAR conversion blocks.

</div>
</article>

<article class="doc-entry" id="v-json-uint64-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-uint64-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-json-uint64-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1688" title="View source at line 1688">Source</a></span></div>
<div class="doc-body">

Full-range unsigned64 numbers with checked parsing and SWAR decimal blocks.

</div>
</article>

<article class="doc-entry" id="v-json-uint8-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-uint8-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-json-uint8-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1742" title="View source at line 1742">Source</a></span></div>
<div class="doc-body">

Unsigned eight-bit integers with exact range refusal.

</div>
</article>

<article class="doc-entry" id="v-json-unmarshal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-unmarshal</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-unmarshal" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L543" title="View source at line 543">Source</a></span></div>
<div class="doc-body">

Decode into a fresh default value, rejecting syntax and conversion failures.

</div>
</article>

<article class="doc-entry" id="v-json-unmarshal-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-unmarshal-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-unmarshal-with" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L539" title="View source at line 539">Source</a></span></div>
<div class="doc-body">

Decode into a fresh default value with explicit conversion policies.

</div>
</article>

<article class="doc-entry" id="v-json-unsupported-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-unsupported-key</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonKey" title="Text.Json.Codec.JsonKey">JsonKey</a> <span class="tok-variable">k</span></code><span class="doc-sig-links"><a href="#v-json-unsupported-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L2049" title="View source at line 2049">Source</a></span></div>
<div class="doc-body">

Declare an application key family that has no JSON conversion capability.

</div>
</article>

<article class="doc-entry" id="v-json-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-update</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L535" title="View source at line 535">Source</a></span></div>
<div class="doc-body">

Decode into an existing application value with established defaults and partial-update reporting.

</div>
</article>

<article class="doc-entry" id="v-json-update-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-update-value</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-update-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L370" title="View source at line 370">Source</a></span></div>
<div class="doc-body">

Apply a retained source value through its immutable typed update capability.

</div>
</article>

<article class="doc-entry" id="v-json-update-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-update-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonReadOptions" title="Text.Json.Codec.JsonReadOptions">JsonReadOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonDecoded" title="Text.Json.Codec.JsonDecoded">JsonDecoded</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-update-with" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L487" title="View source at line 487">Source</a></span></div>
<div class="doc-body">

Apply policy-selected syntax validation while retaining advanced values and the first original refusal.

</div>
</article>

<article class="doc-entry" id="v-json-usize-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-usize-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a></code><span class="doc-sig-links"><a href="#v-json-usize-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L1751" title="View source at line 1751">Source</a></span></div>
<div class="doc-body">

Target-width unsigned integers with platform-specific checked bounds.

</div>
</article>

<article class="doc-entry" id="v-json-write-addressable">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-addressable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-write-addressable" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L216" title="View source at line 216">Source</a></span></div>
<div class="doc-body">

Whether the current application value permits an addressable-only encoding method.

</div>
</article>

<article class="doc-entry" id="v-json-write-html">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-html</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-write-html" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L188" title="View source at line 188">Source</a></span></div>
<div class="doc-body">

Whether string and raw fragments escape HTML-sensitive characters.

</div>
</article>

<article class="doc-entry" id="v-json-write-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a></code><span class="doc-sig-links"><a href="#v-json-write-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L180" title="View source at line 180">Source</a></span></div>
<div class="doc-body">

Construct explicit options; pretty mode remains distinct from an empty indentation string.

</div>
</article>

<article class="doc-entry" id="v-json-write-policies">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-policies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonPolicies" title="Text.Json.Codec.JsonPolicies">JsonPolicies</a></code><span class="doc-sig-links"><a href="#v-json-write-policies" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L204" title="View source at line 204">Source</a></span></div>
<div class="doc-body">

Typed conversion choices used while constructing a value fragment.

</div>
</article>

<article class="doc-entry" id="v-json-write-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-write-prefix" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L196" title="View source at line 196">Source</a></span></div>
<div class="doc-body">

Prefix inserted before each continuation line.

</div>
</article>

<article class="doc-entry" id="v-json-write-pretty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-pretty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-write-pretty" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L192" title="View source at line 192">Source</a></span></div>
<div class="doc-body">

Whether a final whole-value indentation pass is requested.

</div>
</article>

<article class="doc-entry" id="v-json-write-step">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-step</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-write-step" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L200" title="View source at line 200">Source</a></span></div>
<div class="doc-body">

Indentation inserted once per container depth.

</div>
</article>

<article class="doc-entry" id="v-json-write-stringify-numbers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-stringify-numbers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-write-stringify-numbers" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L208" title="View source at line 208">Source</a></span></div>
<div class="doc-body">

Whether numeric descriptors encode their values as JSON strings in this nested context.

</div>
</article>

<article class="doc-entry" id="v-json-write-with-addressability">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-with-addressability</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a></code><span class="doc-sig-links"><a href="#v-json-write-with-addressability" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L221" title="View source at line 221">Source</a></span></div>
<div class="doc-body">

Select an explicit application reference boundary while retaining every other encoding option.
This metadata permits method selection without exposing an application address.

</div>
</article>

<article class="doc-entry" id="v-json-write-with-stringify-numbers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-with-stringify-numbers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonWriteOptions" title="Text.Json.Codec.JsonWriteOptions">JsonWriteOptions</a></code><span class="doc-sig-links"><a href="#v-json-write-with-stringify-numbers" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Codec/source/#L212" title="View source at line 212">Source</a></span></div>
<div class="doc-body">

Select nested numeric quoting without changing HTML or indentation configuration.

</div>
</article>
