# Text.Json.Dynamic

Owned dynamic JSON trees, separate encoding and decoding descriptors, typed payloads and bounded construction.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Json.Dynamic</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Json.Dynamic/source/">View source</a><a href="/packages/search/?q=%2BText.Json.Dynamic">Search module</a><a href="/reference-sources/Text.Json.Dynamic.cinch" download="Text.Json.Dynamic.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Json.Codec](Text.Json.Codec.md), [Text.Json.Value](Text.Json.Value.md).

<details class="doc-notes"><summary>Module notes</summary>

Owned dynamic JSON values with exact number spellings and explicit container identity.
Array and object spines are private. Traversals use explicit work frames so
nesting consumes managed storage rather than one native activation per level.
Raw application strings and names retain exact bytes; encoding repairs malformed
UTF-8 before ordering names. Decoded objects retain the last occurrence of a name.
Explicit typed payloads retain their application value and descriptor capability;
arbitrary mutable application storage follows that descriptor's ownership contract.

</details>

<details class="doc-synopsis"><summary>Synopsis (24 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-JsonAny">JsonAny</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonAny</strong></code></li><li><a class="syn-name" href="#v-json-any-array">json-any-array</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-array</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-array-values">json-any-array-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-array-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a>)</code></li><li><a class="syn-name" href="#v-json-any-as-bool">json-any-as-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-as-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-any-as-bytes">json-any-as-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-as-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-any-as-float64">json-any-as-float64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-as-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-json-any-as-number">json-any-as-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-as-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code></li><li><a class="syn-name" href="#v-json-any-as-string">json-any-as-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-as-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-any-bool">json-any-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-bytes">json-any-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-container-is-null">json-any-container-is-null</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-container-is-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-any-float64">json-any-float64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-is-null">json-any-is-null</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-is-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-any-kind">json-any-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li><a class="syn-name" href="#v-json-any-null">json-any-null</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-null-array">json-any-null-array</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-null-array</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-null-object">json-any-null-object</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-null-object</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-number">json-any-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-object">json-any-object</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-object</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-object-members">json-any-object-members</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-object-members</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a>))</code></li><li><a class="syn-name" href="#v-json-any-string">json-any-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-type">json-any-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-typed">json-any-typed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-typed</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-typed-label">json-any-typed-label</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-typed-label</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-JsonAny">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonAny</strong></code><span class="doc-sig-links"><a href="#t-JsonAny" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

Opaque dynamic payloads with direct recursive container spines.

</div>
</article>

# Values

<article class="doc-entry" id="v-json-any-array">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-array</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-array" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Construct a present array in application order, retaining immutable child owners.

</div>
</article>

<article class="doc-entry" id="v-json-any-array-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-array-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a>)</code><span class="doc-sig-links"><a href="#v-json-any-array-values" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L180" title="View source at line 180">Source</a></span></div>
<div class="doc-body">

Inspect array items; a typed null array is represented by a present empty item list.

</div>
</article>

<article class="doc-entry" id="v-json-any-as-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-as-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-any-as-bool" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L156" title="View source at line 156">Source</a></span></div>
<div class="doc-body">

Inspect a present boolean, including false.

</div>
</article>

<article class="doc-entry" id="v-json-any-as-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-as-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-any-as-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L168" title="View source at line 168">Source</a></span></div>
<div class="doc-body">

Inspect independently copied exact application string bytes.

</div>
</article>

<article class="doc-entry" id="v-json-any-as-float64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-as-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-json-any-as-float64" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

Inspect a floating payload without coercing an exact-number payload.

</div>
</article>

<article class="doc-entry" id="v-json-any-as-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-as-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code><span class="doc-sig-links"><a href="#v-json-any-as-number" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L164" title="View source at line 164">Source</a></span></div>
<div class="doc-body">

Inspect an exact-number payload without formatting floating values.

</div>
</article>

<article class="doc-entry" id="v-json-any-as-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-as-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-any-as-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L172" title="View source at line 172">Source</a></span></div>
<div class="doc-body">

Project application string bytes to scalars, replacing each malformed byte with U+FFFD.

</div>
</article>

<article class="doc-entry" id="v-json-any-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-bool" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Construct a present boolean application value.

</div>
</article>

<article class="doc-entry" id="v-json-any-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

Snapshot arbitrary application string bytes, preserving malformed sequences until encoding.

</div>
</article>

<article class="doc-entry" id="v-json-any-container-is-null">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-container-is-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-any-container-is-null" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L152" title="View source at line 152">Source</a></span></div>
<div class="doc-body">

Distinguish a present typed null collection from a present empty collection.

</div>
</article>

<article class="doc-entry" id="v-json-any-float64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-float64" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Construct a binary64 payload; encoding reports nonfinite values explicitly.

</div>
</article>

<article class="doc-entry" id="v-json-any-is-null">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-is-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-any-is-null" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L148" title="View source at line 148">Source</a></span></div>
<div class="doc-body">

Only an absent dynamic payload is zero or empty for application-field omission.

</div>
</article>

<article class="doc-entry" id="v-json-any-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code><span class="doc-sig-links"><a href="#v-json-any-kind" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

Inspect known representation categories; an opaque typed callback has the unspecified category until encoded.

</div>
</article>

<article class="doc-entry" id="v-json-any-null">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-null" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Construct the absent dynamic application value.

</div>
</article>

<article class="doc-entry" id="v-json-any-null-array">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-null-array</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-null-array" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

A typed null array remains a present dynamic payload and marshals as null.

</div>
</article>

<article class="doc-entry" id="v-json-any-null-object">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-null-object</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-null-object" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

A typed null object remains a present dynamic payload and marshals as null.

</div>
</article>

<article class="doc-entry" id="v-json-any-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-number" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L73" title="View source at line 73">Source</a></span></div>
<div class="doc-body">

Construct an exact-number payload without silently converting it to binary64.

</div>
</article>

<article class="doc-entry" id="v-json-any-object">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-object</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-object" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

Construct an object with exact byte keys in lexical order; later equal keys replace earlier values.

</div>
</article>

<article class="doc-entry" id="v-json-any-object-members">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-object-members</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a>))</code><span class="doc-sig-links"><a href="#v-json-any-object-members" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L192" title="View source at line 192">Source</a></span></div>
<div class="doc-body">

Inspect exact application object names in raw-byte order, each returned name independently owned.

</div>
</article>

<article class="doc-entry" id="v-json-any-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

Construct a present scalar string with independently owned UTF-8 bytes.

</div>
</article>

<article class="doc-entry" id="v-json-any-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L384" title="View source at line 384">Source</a></span></div>
<div class="doc-body">

Dynamic descriptor: absent-only omission, exact-number selection and iterative owned traversal.

</div>
</article>

<article class="doc-entry" id="v-json-any-typed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-typed</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-typed" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L96" title="View source at line 96">Source</a></span></div>
<div class="doc-body">

Retain an application value and its descriptor for use inside mixed dynamic containers.
Encoding invokes the descriptor with the current options. A typed payload is
present for omission, including a typed value whose descriptor emits null.
Mutable application data follows the supplied descriptor's aliasing contract.

</div>
</article>

<article class="doc-entry" id="v-json-any-typed-label">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-typed-label</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-any-typed-label" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Dynamic/source/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

Inspect an opaque typed payload's label without fabricating a decoded representation.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Json/Dynamic.cinch`](/packages/stdlib/Text.Json.Dynamic/source/). Source SHA-256: `19b7ea13c5bc726c4d3ece8b12913a427d0c9ffcdd17d6fa971b9f94149b6591`.
