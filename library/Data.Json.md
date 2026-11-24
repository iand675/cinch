# Data.Json

The JSON value representation shared by parsers, renderers, codecs, and structured observability.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Data.Json</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Json/source/">View source</a><a href="/packages/search/?q=%2BData.Json">Search module</a><a href="/reference-sources/Data.Json.cinch" download="Data.Json.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Json: the `Json`-shaped value a file converter binds.

With no declared type a JSON file binds this shape; with a declared
record type the converter decodes into it instead (surface grammar
local amendments section 11). An array is a cons spine (`JArr item
tail`, ended by `JArrEnd`) and an object entry is `JObj key value
tail` (ended by `JObjEnd`): the positivity discipline allows only
direct recursive occurrences, so the payloads stay inside the
family.

</details>

<details class="doc-synopsis" open><summary>Synopsis (1 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Json">Json</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Json</strong></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Json">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Json</strong></code><span class="doc-sig-links"><a href="#t-Json" title="Permalink">#</a><a href="/packages/stdlib/Data.Json/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

The `Json`-shaped value a file converter binds for a JSON file with
no declared type (surface grammar local amendments section 11); with
a declared record type the converter decodes into it instead. An
array is a cons spine (`JArr item tail`, ended by `JArrEnd`) and an
object entry is `JObj key value tail` (ended by `JObjEnd`): the
positivity discipline allows only direct recursive occurrences, so
the payloads stay inside the family.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JNull"><code><strong class="tok-constructor">JNull</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JBool"><code><strong class="tok-constructor">JBool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JNum"><code><strong class="tok-constructor">JNum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JFloat"><code><strong class="tok-constructor">JFloat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JStr"><code><strong class="tok-constructor">JStr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JArr"><code><strong class="tok-constructor">JArr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JObj"><code><strong class="tok-constructor">JObj</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JArrEnd"><code><strong class="tok-constructor">JArrEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JObjEnd"><code><strong class="tok-constructor">JObjEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.Json/#instances">Text.Json</a></span></li></ul></details>
</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Json.cinch`](/packages/stdlib/Data.Json/source/). Source SHA-256: `6af8ff9d91b25b53c034b1626efe76bf1a6e277172920eabbd8c21e4c0781e3b`.
