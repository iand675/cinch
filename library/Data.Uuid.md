# Data.Uuid

Immutable 128-bit identifiers with RFC 9562 text, ordering and explicit version 7 state.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Uuid</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Uuid/source/">View source</a><a href="/packages/search/?q=%2BData.Uuid">Search module</a><a href="/reference-sources/Data.Uuid.cinch" download="Data.Uuid.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Core](Data.Core.md), [Data.Hash](Data.Hash.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Ordering](Data.Ordering.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Hex](Text.Hex.md).

<details class="doc-notes"><summary>Module notes</summary>

Immutable 128-bit identifiers with RFC 9562 text, ordering and explicit version 7 state.
Every bit pattern is a valid value. Parsing validates only the accepted text
forms; version and variant bits are set only by generation functions. Values
contain two network-order words and never retain a mutable byte-buffer alias.

</details>

<details class="doc-synopsis"><summary>Synopsis (24 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ParsedUuid">ParsedUuid</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ParsedUuid</strong></code></li><li><a class="syn-name" href="#t-Uuid">Uuid</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Uuid</strong></code></li><li><a class="syn-name" href="#t-UuidError">UuidError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UuidError</strong></code></li><li><a class="syn-name" href="#t-UuidV7State">UuidV7State</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UuidV7State</strong></code></li><li><a class="syn-name" href="#v-uuid-append-text">uuid-append-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-append-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-uuid-compare">uuid-compare</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-uuid-equal">uuid-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-uuid-format">uuid-format</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-format</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-uuid-from-bytes">uuid-from-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-from-words">uuid-from-words</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-from-words</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-marshal-text">uuid-marshal-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-marshal-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-uuid-max">uuid-max</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-max</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-nil">uuid-nil</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-nil</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-parse">uuid-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-parse-bytes">uuid-parse-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-parse-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-required">uuid-required</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-ParsedUuid" title="Data.Uuid.ParsedUuid">ParsedUuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-to-bytes">uuid-to-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-uuid-unmarshal-text">uuid-unmarshal-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-unmarshal-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a>)</code></li><li><a class="syn-name" href="#v-uuid-v4-from-random">uuid-v4-from-random</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-v4-from-random</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-v7-advance">uuid-v7-advance</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-v7-advance</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidV7State" title="Data.Uuid.UuidV7State">UuidV7State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidV7State" title="Data.Uuid.UuidV7State">UuidV7State</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>)</code></li><li><a class="syn-name" href="#v-uuid-v7-from-tick">uuid-v7-from-tick</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-v7-from-tick</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-v7-initial">uuid-v7-initial</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-v7-initial</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidV7State" title="Data.Uuid.UuidV7State">UuidV7State</a></code></li><li><a class="syn-name" href="#v-uuid-v7-step">uuid-v7-step</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-v7-step</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidV7State" title="Data.Uuid.UuidV7State">UuidV7State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidV7State" title="Data.Uuid.UuidV7State">UuidV7State</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a>)</code></li><li><a class="syn-name" href="#v-uuid-validate-text">uuid-validate-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-validate-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-ParsedUuid" title="Data.Uuid.ParsedUuid">ParsedUuid</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ParsedUuid">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ParsedUuid</strong></code><span class="doc-sig-links"><a href="#t-ParsedUuid" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

A successfully checked text value, retaining the decoded identifier rather than its input buffer.

</div>
</article>

<article class="doc-entry" id="t-Uuid">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Uuid</strong></code><span class="doc-sig-links"><a href="#t-Uuid" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

Two unsigned words in network significance order, with no mutable backing store.

<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Uuid/#instances">Data.Uuid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Uuid/#instances">Data.Uuid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Uuid/#instances">Data.Uuid</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-UuidError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UuidError</strong></code><span class="doc-sig-links"><a href="#t-UuidError" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

A checked text, buffer, time or logical-clock refusal.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UuidInvalidLength"><code><strong class="tok-constructor">UuidInvalidLength</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a></code></li><li id="k-UuidInvalidText"><code><strong class="tok-constructor">UuidInvalidText</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a></code></li><li id="k-UuidOutputTooLarge"><code><strong class="tok-constructor">UuidOutputTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a></code></li><li id="k-UuidClockOutOfRange"><code><strong class="tok-constructor">UuidClockOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a></code></li><li id="k-UuidClockExhausted"><code><strong class="tok-constructor">UuidClockExhausted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UuidV7State">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UuidV7State</strong></code><span class="doc-sig-links"><a href="#t-UuidV7State" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

Explicit version 7 ordering history for one independent generation sequence.

</div>
</article>

# Values

<article class="doc-entry" id="v-uuid-append-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-append-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-uuid-append-text" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L192" title="View source at line 192">Source</a></span></div>
<div class="doc-body">

Copy an arbitrary prefix and append canonical text without exposing mutable aliases or partial output.

</div>
</article>

<article class="doc-entry" id="v-uuid-compare">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-uuid-compare" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

Compare the full 128-bit network order, including reserved and unfamiliar version bits.

</div>
</article>

<article class="doc-entry" id="v-uuid-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-uuid-equal" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

Test complete identifier equality without pointer identity shortcuts.

</div>
</article>

<article class="doc-entry" id="v-uuid-format">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-format</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-uuid-format" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L184" title="View source at line 184">Source</a></span></div>
<div class="doc-body">

Return the canonical 36-byte lowercase string; all generated bytes are valid ASCII.

</div>
</article>

<article class="doc-entry" id="v-uuid-from-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-from-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Read exactly 16 bytes and copy their bits into an immutable value.

</div>
</article>

<article class="doc-entry" id="v-uuid-from-words">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-from-words</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-from-words" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

Construct any identifier value directly from its two network-order words.

</div>
</article>

<article class="doc-entry" id="v-uuid-marshal-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-marshal-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-uuid-marshal-text" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

Encode all bits with existing SIMD hex lanes and fixed lowercase canonical punctuation.

</div>
</article>

<article class="doc-entry" id="v-uuid-max">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-max</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-max" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

The all-one identifier; no version restriction excludes this sentinel value.

</div>
</article>

<article class="doc-entry" id="v-uuid-nil">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-nil</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-nil" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

The all-zero identifier, independent of any optional or absent carrier.

</div>
</article>

<article class="doc-entry" id="v-uuid-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-parse" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L146" title="View source at line 146">Source</a></span></div>
<div class="doc-body">

Parse text atomically; hexadecimal letter case is accepted without validating version or variant bits.

</div>
</article>

<article class="doc-entry" id="v-uuid-parse-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-parse-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-parse-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L134" title="View source at line 134">Source</a></span></div>
<div class="doc-body">

Parse canonical, brace-wrapped, exact lowercase URN or compact ASCII forms without trimming.

</div>
</article>

<article class="doc-entry" id="v-uuid-required">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-ParsedUuid" title="Data.Uuid.ParsedUuid">ParsedUuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-required" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

Extract a previously checked identifier without a partial parser or runtime panic.

</div>
</article>

<article class="doc-entry" id="v-uuid-to-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-uuid-to-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Return a fresh 16-byte network-order buffer; mutating it cannot change the identifier.

</div>
</article>

<article class="doc-entry" id="v-uuid-unmarshal-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-unmarshal-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a>)</code><span class="doc-sig-links"><a href="#v-uuid-unmarshal-text" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L150" title="View source at line 150">Source</a></span></div>
<div class="doc-body">

Replace an immutable receiver on success and preserve every original bit on malformed text.

</div>
</article>

<article class="doc-entry" id="v-uuid-v4-from-random">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-v4-from-random</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-v4-from-random" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L203" title="View source at line 203">Source</a></span></div>
<div class="doc-body">

Set version 4 and RFC variant10 while preserving the other 122 supplied entropy bits.

</div>
</article>

<article class="doc-entry" id="v-uuid-v7-advance">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-v7-advance</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidV7State" title="Data.Uuid.UuidV7State">UuidV7State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidV7State" title="Data.Uuid.UuidV7State">UuidV7State</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>)</code><span class="doc-sig-links"><a href="#v-uuid-v7-advance" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L214" title="View source at line 214">Source</a></span></div>
<div class="doc-body">

Advance the 60-bit timestamp, retaining same-second rollback ordering and resetting only on whole-second rollback.

</div>
</article>

<article class="doc-entry" id="v-uuid-v7-from-tick">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-v7-from-tick</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-v7-from-tick" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L229" title="View source at line 229">Source</a></span></div>
<div class="doc-body">

Combine a checked logical timestamp with eight supplied random bytes, retaining 62 random bits.

</div>
</article>

<article class="doc-entry" id="v-uuid-v7-initial">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-v7-initial</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidV7State" title="Data.Uuid.UuidV7State">UuidV7State</a></code><span class="doc-sig-links"><a href="#v-uuid-v7-initial" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L210" title="View source at line 210">Source</a></span></div>
<div class="doc-body">

Begin one independent version 7 sequence without a hidden shared process-global counter.

</div>
</article>

<article class="doc-entry" id="v-uuid-v7-step">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-v7-step</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidV7State" title="Data.Uuid.UuidV7State">UuidV7State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidV7State" title="Data.Uuid.UuidV7State">UuidV7State</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a>)</code><span class="doc-sig-links"><a href="#v-uuid-v7-step" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L240" title="View source at line 240">Source</a></span></div>
<div class="doc-body">

Run a pure version 7 transition with explicit time and entropy; refusal returns no replacement state.

</div>
</article>

<article class="doc-entry" id="v-uuid-validate-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-validate-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-ParsedUuid" title="Data.Uuid.ParsedUuid">ParsedUuid</a></code><span class="doc-sig-links"><a href="#v-uuid-validate-text" title="Permalink">#</a><a href="/packages/stdlib/Data.Uuid/source/#L156" title="View source at line 156">Source</a></span></div>
<div class="doc-body">

Check a trusted-text witness explicitly, retaining ordinary parse errors for invalid input.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Uuid"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Uuid.Eq_Uuid</code></span><div class="doc-child-prose">Complete-bit equality for identifier values.</div></li>
<li id="i-Hash_Uuid"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Uuid.Hash_Uuid</code></span><div class="doc-child-prose">Hash both complete words; hash collisions never replace identifier equality.</div></li>
<li id="i-Ord_Uuid"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Uuid.Ord_Uuid</code></span><div class="doc-child-prose">Unsigned network-order comparison, consistent with complete-bit equality.</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Uuid.cinch`](/packages/stdlib/Data.Uuid/source/). Source SHA-256: `d36bcfabe392c1daba83c765280fbdead78126aa6190e0f472af6fc3bd8bd87c`.
