# Text.Json.Binary32

Exact decimal rounding and shortest finite binary32 JSON coefficients, with preserved signed zero and midpoint comparisons.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Json.Binary32</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Json.Binary32/source/">View source</a><a href="/packages/search/?q=%2BText.Json.Binary32">Search module</a><a href="/reference-sources/Text.Json.Binary32.cinch" download="Text.Json.Binary32.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Exact decimal rounding and shortest JSON coefficients for finite binary32 values.
Parsing consumes an already validated decimal spelling. Decimal midpoint
comparisons repair binary64 double rounding without accumulating input bignums.

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-binary32-from-decimal">binary32-from-decimal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary32-from-decimal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code></li><li><a class="syn-name" href="#v-binary32-to-decimal">binary32-to-decimal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">binary32-to-decimal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-binary32-from-decimal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary32-from-decimal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code><span class="doc-sig-links"><a href="#v-binary32-from-decimal" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Binary32/source/#L118" title="View source at line 118">Source</a></span></div>
<div class="doc-body">

Round a validated nonempty JSON decimal exactly to binary32, including signed underflow and overflow.

</div>
</article>

<article class="doc-entry" id="v-binary32-to-decimal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">binary32-to-decimal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-binary32-to-decimal" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Binary32/source/#L189" title="View source at line 189">Source</a></span></div>
<div class="doc-body">

Format a finite binary32 as its shortest exact JSON decimal, preserving negative zero.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Json/Binary32.cinch`](/packages/stdlib/Text.Json.Binary32/source/). Source SHA-256: `ad2ac932fc7da1befe03fefc9fdf84b98c7276f7af996ed9d3617bfd635d2693`.
