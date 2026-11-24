# Effect.Buffered.Internal.Scan

Exact bounded byte scans shared by buffered input and token splitters.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Buffered.Internal.Scan</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Buffered.Internal.Scan/source/">View source</a><a href="/packages/search/?q=%2BEffect.Buffered.Internal.Scan">Search module</a><a href="/reference-sources/Effect.Buffered.Internal.Scan.cinch" download="Effect.Buffered.Internal.Scan.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

<details class="doc-notes"><summary>Module notes</summary>

Exact bounded byte scans shared by buffered input and token splitters.
Complete eight-byte words use lane comparisons; scalar fringes never read
before the requested start or after the half-open end of the owned span.

</details>

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-ascii-spaces">ascii-spaces</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii-spaces</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ascii-word">ascii-word</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-find-byte">find-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">find-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-ascii-spaces">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii-spaces</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ascii-spaces" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered.Internal.Scan/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Advance past ASCII whitespace in a bounded span, returning an absolute endpoint; invalid bounds leave the offset unchanged.

</div>
</article>

<article class="doc-entry" id="v-ascii-word">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ascii-word" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered.Internal.Scan/source/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Advance past ASCII non-whitespace bytes, stopping before Unicode fringes for scalar decoding.

</div>
</article>

<article class="doc-entry" id="v-find-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">find-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-find-byte" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered.Internal.Scan/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Find a byte in a checked half-open owner span, returning its absolute offset or None for absence or invalid bounds.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Buffered/Internal/Scan.cinch`](/packages/stdlib/Effect.Buffered.Internal.Scan/source/). Source SHA-256: `027cfc4371a5d8d9f876582c684035fe180b8158e8aacc85a67efeee3f67153e`.
