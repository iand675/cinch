# Data.Bytes.Search

Exact first, last, containment and count queries over raw bytes with bounded SWAR traversals.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Bytes.Search</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Bytes.Search/source/">View source</a><a href="/packages/search/?q=%2BData.Bytes.Search">Search module</a><a href="/reference-sources/Data.Bytes.Search.cinch" download="Data.Bytes.Search.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

<details class="doc-notes"><summary>Module notes</summary>

Exact raw-byte searching with bounded machine loops and broad-word classification.
Complete eight-byte blocks use private straight-line SWAR kernels. Scalar
fringes read only their available bytes. Public results are zero-based byte
offsets; every byte value, including malformed UTF-8, is compared exactly.
Backing owners remain live around complete private traversals.

</details>

<details class="doc-synopsis" open><summary>Synopsis (4 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-bytes-contains-byte">bytes-contains-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-contains-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-bytes-count-byte">bytes-count-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-count-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-bytes-index-byte">bytes-index-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-index-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-bytes-last-index-byte">bytes-last-index-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-last-index-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-bytes-contains-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-contains-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-bytes-contains-byte" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Search/source/#L232" title="View source at line 232">Source</a></span></div>
<div class="doc-body">

Test whether any exact byte value occurs, including zero and high-bit byte values.

</div>
</article>

<article class="doc-entry" id="v-bytes-count-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-count-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-bytes-count-byte" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Search/source/#L239" title="View source at line 239">Source</a></span></div>
<div class="doc-body">

Count every exact matching byte using eight-lane masks and a bounded scalar fringe.

</div>
</article>

<article class="doc-entry" id="v-bytes-index-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-index-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-bytes-index-byte" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Search/source/#L218" title="View source at line 218">Source</a></span></div>
<div class="doc-body">

Find the first exact byte value, returning no offset when the input contains no match.

</div>
</article>

<article class="doc-entry" id="v-bytes-last-index-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-last-index-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-bytes-last-index-byte" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Search/source/#L225" title="View source at line 225">Source</a></span></div>
<div class="doc-body">

Find the last exact byte value without decoding or changing the input.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Bytes/Search.cinch`](/packages/stdlib/Data.Bytes.Search/source/). Source SHA-256: `65b8c75745399b692031aff515386ffaed14ad9f50b4e4a644a8f3ce259c5256`.
