# Data.Bytes.Build

Fresh byte cloning, doubling repetition, concatenation and separator-aware joining.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Bytes.Build</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Bytes.Build/source/">View source</a><a href="/packages/search/?q=%2BData.Bytes.Build">Search module</a><a href="/reference-sources/Data.Bytes.Build.cinch" download="Data.Bytes.Build.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md).

<details class="doc-notes"><summary>Module notes</summary>

Bulk construction of owned mutable byte buffers with exact single-allocation results.
Clone, repetition, concatenation and joining never retain caller input aliases.
Complete copies use the runtime's accelerated checked range-copy operation;
list traversal and repeated-prefix expansion use ordinary bounded-stack loops.

</details>

<details class="doc-synopsis" open><summary>Synopsis (4 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-bytes-clone">bytes-clone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-bytes-concat">bytes-concat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-concat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-bytes-join">bytes-join</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-bytes-repeat">bytes-repeat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-repeat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-bytes-clone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-bytes-clone" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Build/source/#L15" title="View source at line 15">Source</a></span></div>
<div class="doc-body">

Copy every source byte into independent mutable storage, including empty input.

</div>
</article>

<article class="doc-entry" id="v-bytes-concat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-concat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-bytes-concat" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Build/source/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

Concatenate raw byte pieces into one fresh buffer without recursive return frames.

</div>
</article>

<article class="doc-entry" id="v-bytes-join">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-bytes-join" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Build/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

Join raw byte pieces in list order with separators, into one fresh exact-size buffer.
Empty pieces still contribute their adjacent separators; caller buffers remain unchanged.

</div>
</article>

<article class="doc-entry" id="v-bytes-repeat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-repeat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-bytes-repeat" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Build/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

Repeat raw bytes the requested natural count into one independent buffer.
Empty inputs and zero counts need no fill traversal; byte values are never decoded.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Bytes/Build.cinch`](/packages/stdlib/Data.Bytes.Build/source/). Source SHA-256: `6c71c2b3807a54c6f168ea5acf4aa1f2486801501487f9494e6e18a8d9449de9`.
