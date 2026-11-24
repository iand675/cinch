# Text.Json.Legacy

Explicit legacy codec policy and descriptor bridges over owned dynamic values.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Json.Legacy</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Json.Legacy/source/">View source</a><a href="/packages/search/?q=%2BText.Json.Legacy">Search module</a><a href="/reference-sources/Text.Json.Legacy.cinch" download="Text.Json.Legacy.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Codec](Text.Codec.md), [Text.Json.Codec](Text.Json.Codec.md), [Text.Json.Dynamic](Text.Json.Dynamic.md), [Text.Json.Value](Text.Json.Value.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (11 files)</summary>

Copy these modules beside your entry file before importing `Text.Json.Legacy`: [Data.Array](Data.Array.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Json.Binary32](Text.Json.Binary32.md), [Text.Json.Codec](Text.Json.Codec.md), [Text.Json.Dynamic](Text.Json.Dynamic.md), [Text.Json.Legacy](Text.Json.Legacy.md), [Text.Json.Value](Text.Json.Value.md).

```sh
cp seed/prelude/Data/Array.cinch ./Data.Array.cinch
cp seed/prelude/Data/Bytes/Utf8.cinch ./Data.Bytes.Utf8.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Data/Unicode/Kernel.cinch ./Data.Unicode.Kernel.cinch
cp seed/prelude/Data/Unicode/Tables.cinch ./Data.Unicode.Tables.cinch
cp seed/prelude/Text/Base64/Encoding.cinch ./Text.Base64.Encoding.cinch
cp seed/prelude/Text/Json/Binary32.cinch ./Text.Json.Binary32.cinch
cp seed/prelude/Text/Json/Codec.cinch ./Text.Json.Codec.cinch
cp seed/prelude/Text/Json/Dynamic.cinch ./Text.Json.Dynamic.cinch
cp seed/prelude/Text/Json/Legacy.cinch ./Text.Json.Legacy.cinch
cp seed/prelude/Text/Json/Value.cinch ./Text.Json.Value.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Explicit bridges between dynamic JSON and the established Json-shaped codec family.
Integer spellings become arbitrary-precision integers; fractional or exponent
spellings become finite binary64 values. Exact-number spelling and typed null
collection identity are intentionally projected by this explicit conversion.
Malformed application text, invalid numbers and malformed legacy spines are
refused instead of being silently repaired. Existing legacy APIs are unchanged.
Opaque typed application payloads are projected through their encoding capability
once, then decoded with exact number spellings before the established projection.

</details>

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-json-any-from-legacy">json-any-from-legacy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-from-legacy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code></li><li><a class="syn-name" href="#v-json-any-to-legacy">json-any-to-legacy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-any-to-legacy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-json-legacy-type">json-legacy-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-legacy-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">codec</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-json-any-from-legacy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-from-legacy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a></code><span class="doc-sig-links"><a href="#v-json-any-from-legacy" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Legacy/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Convert established Json-shaped values to owned dynamic values; later duplicate object names replace earlier values.
Nonfinite floating payloads remain present and are refused only if subsequently
marshaled by the strict dynamic descriptor. Malformed container tails are errors.

</div>
</article>

<article class="doc-entry" id="v-json-any-to-legacy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-any-to-legacy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Dynamic/#t-JsonAny" title="Text.Json.Dynamic.JsonAny">JsonAny</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-json-any-to-legacy" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Legacy/source/#L158" title="View source at line 158">Source</a></span></div>
<div class="doc-body">

Explicitly project dynamic values into the established Json shape with named representability refusals.
Integer precision and floating payload bits are retained. Exact fractional
spelling becomes a finite binary64 value; typed null collections become JNull.

</div>
</article>

<article class="doc-entry" id="v-json-legacy-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-legacy-type</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">codec</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Codec/#t-JsonType" title="Text.Json.Codec.JsonType">JsonType</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-legacy-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Legacy/source/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

Adapt an existing Json-shaped codec using explicit labels, defaults and application omission classifiers.
This is an explicit capability adapter rather than a global change to either
codec family. Refused decoding retains the supplied prior application value.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Json/Legacy.cinch`](/packages/stdlib/Text.Json.Legacy/source/). Source SHA-256: `c486fb1b879d656fb5a61b1e5d2b098563e269a3d53a273cc1b9f50198fba826`.
