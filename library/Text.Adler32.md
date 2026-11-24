# Text.Adler32

Portable SIMD Adler-32 with bounded machine-word segments, persistent states, network-order digests and checked versioned checkpoints.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Adler32</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Adler32/">View source</a><a href="/library/search/?q=%2BText.Adler32">Search module</a><a href="/reference-sources/Text.Adler32.cinch" download="Text.Adler32.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

<details class="doc-closure"><summary>Ship-source closure (3 files)</summary>

Copy these modules beside your entry file before importing `Text.Adler32`: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Adler32](Text.Adler32.md), [Text.Binary](Text.Binary.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Text/Adler32.cinch ./Text.Adler32.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Adler-32 checksums, persistent accumulators and versioned binary checkpoints.
Updates consume sixteen bytes with SIMD weighted reductions and normalize
the two sums after segments of at most 5552 bytes. Accumulators use explicit
32-bit wrapping; normal states satisfy the segment's no-overflow bound.
Digest bytes use network order. Retaining a state branches the computation,
and all returned byte buffers are fresh. This checksum does not authenticate data.

</details>

<details class="doc-synopsis" open><summary>Synopsis (16 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Adler32State">Adler32State</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Adler32State</strong></code></li><li><a class="syn-name" href="#t-Adler32StateError">Adler32StateError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Adler32StateError</strong></code></li><li><a class="syn-name" href="#v-adler32-state">adler32-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a></code></li><li><a class="syn-name" href="#v-adler32-state-append">adler32-state-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-adler32-state-append-save">adler32-state-append-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-adler32-state-block-size">adler32-state-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-adler32-state-error-message">adler32-state-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32StateError" title="Text.Adler32.Adler32StateError">Adler32StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-adler32-state-load">adler32-state-load</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32StateError" title="Text.Adler32.Adler32StateError">Adler32StateError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a></code></li><li><a class="syn-name" href="#v-adler32-state-reset">adler32-state-reset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a></code></li><li><a class="syn-name" href="#v-adler32-state-save">adler32-state-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-adler32-state-size">adler32-state-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-adler32-state-sum">adler32-state-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-adler32-state-update">adler32-state-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a></code></li><li><a class="syn-name" href="#v-adler32-state-word">adler32-state-word</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-state-word</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-adler32-sum">adler32-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-sum</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-adler32-update">adler32-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Adler32State">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Adler32State</strong></code><span class="doc-sig-links"><a href="#t-Adler32State" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

An immutable scalar checksum accumulator; input buffers are never retained.

</div>
</article>

<article class="doc-entry" id="t-Adler32StateError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Adler32StateError</strong></code><span class="doc-sig-links"><a href="#t-Adler32StateError" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

A checkpoint identifier or complete byte length is invalid.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidAdler32StateIdentifier"><code><strong class="tok-constructor">InvalidAdler32StateIdentifier</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32StateError" title="Text.Adler32.Adler32StateError">Adler32StateError</a></code></li><li id="k-InvalidAdler32StateSize"><code><strong class="tok-constructor">InvalidAdler32StateSize</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32StateError" title="Text.Adler32.Adler32StateError">Adler32StateError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-adler32-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a></code><span class="doc-sig-links"><a href="#v-adler32-state" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L72" title="View source at line 72">Source</a></span></div>
<div class="doc-body">

Initial accumulator: low sum 1 and high sum 0.

</div>
</article>

<article class="doc-entry" id="v-adler32-state-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-adler32-state-append" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

Append the network-order digest to a copied prefix without changing either argument.

</div>
</article>

<article class="doc-entry" id="v-adler32-state-append-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-adler32-state-append-save" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

Append a complete binary checkpoint to a copied prefix, preserving the state.

</div>
</article>

<article class="doc-entry" id="v-adler32-state-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-adler32-state-block-size" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

Preferred input granularity in bytes, always four; updates accept any chunk length.

</div>
</article>

<article class="doc-entry" id="v-adler32-state-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32StateError" title="Text.Adler32.Adler32StateError">Adler32StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-adler32-state-error-message" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

Render a checkpoint diagnostic with the complete required and actual lengths.

</div>
</article>

<article class="doc-entry" id="v-adler32-state-load">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32StateError" title="Text.Adler32.Adler32StateError">Adler32StateError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a></code><span class="doc-sig-links"><a href="#v-adler32-state-load" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L134" title="View source at line 134">Source</a></span></div>
<div class="doc-body">

Restore a complete checkpoint without retaining its mutable input buffer.
Identifier checks precede exact-size checks. Every stored word is accepted;
ordinary checksum states should be produced by initialization and updates.

</div>
</article>

<article class="doc-entry" id="v-adler32-state-reset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a></code><span class="doc-sig-links"><a href="#v-adler32-state-reset" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

Return the initial accumulator, leaving the supplied state unchanged.

</div>
</article>

<article class="doc-entry" id="v-adler32-state-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-adler32-state-save" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

Return an eight-byte checkpoint: ASCII adl, version byte 1, and the network-order checksum.

</div>
</article>

<article class="doc-entry" id="v-adler32-state-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-adler32-state-size" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

Digest size in bytes, always four.

</div>
</article>

<article class="doc-entry" id="v-adler32-state-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-adler32-state-sum" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L96" title="View source at line 96">Source</a></span></div>
<div class="doc-body">

Return four fresh checksum bytes in network order without advancing or resetting.

</div>
</article>

<article class="doc-entry" id="v-adler32-state-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a></code><span class="doc-sig-links"><a href="#v-adler32-state-update" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

Consume a complete chunk without changing any earlier retained state or input bytes.

</div>
</article>

<article class="doc-entry" id="v-adler32-state-word">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-state-word</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-adler32-state-word" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

Observe the complete numeric checksum without advancing the state.

</div>
</article>

<article class="doc-entry" id="v-adler32-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-sum</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-adler32-sum" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L24" title="View source at line 24">Source</a></span></div>
<div class="doc-body">

Checksum all input bytes from the standard initial state 1. Empty input yields 1.

</div>
</article>

<article class="doc-entry" id="v-adler32-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-adler32-update" title="Permalink">#</a><a href="/library/source/Text.Adler32/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

Continue a finished checksum with more bytes. Empty input preserves every supplied bit.
Full segments use explicit machine-word wrapping before modular reduction.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Adler32.cinch`](/library/source/Text.Adler32/). Source SHA-256: `30204346a6b8a6d8b0ebe898740eeedcf97c6951ce74ef2b1a52a6bc36f4023c`.
