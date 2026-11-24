# Text.Fnv.State

Persistent FNV states with reset, network-order digests, numeric observations and checked versioned binary checkpoints.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Fnv.State</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Fnv.State/">View source</a><a href="/library/search/?q=%2BText.Fnv.State">Search module</a><a href="/reference-sources/Text.Fnv.State.cinch" download="Text.Fnv.State.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md), [Text.Fnv](Text.Fnv.md).

<details class="doc-closure"><summary>Ship-source closure (4 files)</summary>

Copy these modules beside your entry file before importing `Text.Fnv.State`: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Fnv](Text.Fnv.md), [Text.Fnv.State](Text.Fnv.State.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
cp seed/prelude/Text/Fnv.cinch ./Text.Fnv.cinch
cp seed/prelude/Text/Fnv/State.cinch ./Text.Fnv.State.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Persistent FNV hash states, digest bytes and versioned binary checkpoints.
All six width/order variants retain their identity across updates and reset.
Digests use network byte order. Retaining a state branches a computation:
later updates never change earlier states, and returned buffers are fresh.
Checkpoints contain a four-byte variant identifier followed by the digest;
loading requires a matching variant and exact size. These hashes do not
authenticate data, including their serialized checkpoints.

</details>

<details class="doc-synopsis"><summary>Synopsis (21 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-FnvState">FnvState</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FnvState</strong></code></li><li><a class="syn-name" href="#t-FnvStateError">FnvStateError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FnvStateError</strong></code></li><li><a class="syn-name" href="#v-fnv-state-append">fnv-state-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-fnv-state-append-save">fnv-state-append-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-fnv-state-block-size">fnv-state-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-fnv-state-error-message">fnv-state-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvStateError" title="Text.Fnv.State.FnvStateError">FnvStateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-fnv-state-load">fnv-state-load</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvStateError" title="Text.Fnv.State.FnvStateError">FnvStateError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code></li><li><a class="syn-name" href="#v-fnv-state-reset">fnv-state-reset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code></li><li><a class="syn-name" href="#v-fnv-state-save">fnv-state-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-fnv-state-size">fnv-state-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-fnv-state-sum">fnv-state-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-fnv-state-update">fnv-state-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code></li><li><a class="syn-name" href="#v-fnv-state-word32">fnv-state-word32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-word32</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-fnv-state-word64">fnv-state-word64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-word64</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-fnv-state-words128">fnv-state-words128</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-state-words128</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code></li><li><a class="syn-name" href="#v-fnv128-state">fnv128-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv128-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code></li><li><a class="syn-name" href="#v-fnv128a-state">fnv128a-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv128a-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code></li><li><a class="syn-name" href="#v-fnv32-state">fnv32-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv32-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code></li><li><a class="syn-name" href="#v-fnv32a-state">fnv32a-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv32a-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code></li><li><a class="syn-name" href="#v-fnv64-state">fnv64-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv64-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code></li><li><a class="syn-name" href="#v-fnv64a-state">fnv64a-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv64a-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-FnvState">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FnvState</strong></code><span class="doc-sig-links"><a href="#t-FnvState" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

An immutable hash accumulator with a fixed width and multiplication/xor order.

</div>
</article>

<article class="doc-entry" id="t-FnvStateError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FnvStateError</strong></code><span class="doc-sig-links"><a href="#t-FnvStateError" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

A checkpoint's variant identity or complete byte length is invalid.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidFnvStateIdentifier"><code><strong class="tok-constructor">InvalidFnvStateIdentifier</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvStateError" title="Text.Fnv.State.FnvStateError">FnvStateError</a></code></li><li id="k-InvalidFnvStateSize"><code><strong class="tok-constructor">InvalidFnvStateSize</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvStateError" title="Text.Fnv.State.FnvStateError">FnvStateError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-fnv-state-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-fnv-state-append" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

Append digest bytes to a copied prefix, leaving both the prefix and state unchanged.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-append-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-fnv-state-append-save" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

Append a binary checkpoint to a copied prefix without changing either argument.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-fnv-state-block-size" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Preferred input granularity in bytes. FNV accepts every individual byte.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvStateError" title="Text.Fnv.State.FnvStateError">FnvStateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-fnv-state-error-message" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L148" title="View source at line 148">Source</a></span></div>
<div class="doc-body">

Render a state-loading diagnostic without exposing implementation-specific type names.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-load">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvStateError" title="Text.Fnv.State.FnvStateError">FnvStateError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code><span class="doc-sig-links"><a href="#v-fnv-state-load" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L156" title="View source at line 156">Source</a></span></div>
<div class="doc-body">

Restore a checkpoint of the template's variant. The template and input remain unchanged.
All bit patterns are valid hash states. Identifier checks precede exact-size checks;
loading one variant into another is an identifier error even when their widths agree.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-reset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code><span class="doc-sig-links"><a href="#v-fnv-state-reset" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Return the initial offset basis of the same variant, preserving the original state.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-fnv-state-save" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L133" title="View source at line 133">Source</a></span></div>
<div class="doc-body">

Serialize the variant identifier and all hash bits in fresh network-order bytes.
The identifier is fnv followed by a tag from 1 through 6, ordered by width
and then by FNV-1 versus FNV-1a. Checkpoints have lengths 8, 12 or 20.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-fnv-state-size" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

Number of digest bytes: four, eight or sixteen according to the fixed width.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-fnv-state-sum" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

Return fresh digest bytes in network order without advancing or resetting the state.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code><span class="doc-sig-links"><a href="#v-fnv-state-update" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Continue a state with all input bytes using the ordered native-word kernels.
Input is consumed during this call; no mutable input buffer is retained.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-word32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-word32</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-fnv-state-word32" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Observe a 32-bit state as a machine word; other widths return None.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-word64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-word64</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-fnv-state-word64" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

Observe a 64-bit state as a machine word; other widths return None.

</div>
</article>

<article class="doc-entry" id="v-fnv-state-words128">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-state-words128</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code><span class="doc-sig-links"><a href="#v-fnv-state-words128" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

Observe a 128-bit state's high and low words; other widths return None.

</div>
</article>

<article class="doc-entry" id="v-fnv128-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv128-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code><span class="doc-sig-links"><a href="#v-fnv128-state" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

Initial 128-bit FNV-1 state, retaining both words without truncation.

</div>
</article>

<article class="doc-entry" id="v-fnv128a-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv128a-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code><span class="doc-sig-links"><a href="#v-fnv128a-state" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">

Initial 128-bit FNV-1a state, retaining both words without truncation.

</div>
</article>

<article class="doc-entry" id="v-fnv32-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv32-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code><span class="doc-sig-links"><a href="#v-fnv32-state" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

Initial 32-bit FNV-1 state, multiplying before xor.

</div>
</article>

<article class="doc-entry" id="v-fnv32a-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv32a-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code><span class="doc-sig-links"><a href="#v-fnv32a-state" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

Initial 32-bit FNV-1a state, applying xor before multiplication.

</div>
</article>

<article class="doc-entry" id="v-fnv64-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv64-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code><span class="doc-sig-links"><a href="#v-fnv64-state" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Initial 64-bit FNV-1 state, multiplying before xor.

</div>
</article>

<article class="doc-entry" id="v-fnv64a-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv64a-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code><span class="doc-sig-links"><a href="#v-fnv64a-state" title="Permalink">#</a><a href="/library/source/Text.Fnv.State/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

Initial 64-bit FNV-1a state, applying xor before multiplication.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Fnv/State.cinch`](/library/source/Text.Fnv.State/). Source SHA-256: `32143ed6a2a86719094a7ff3dffcd7811a51d0b7c8aae7630fe1fe0487ecd673`.
