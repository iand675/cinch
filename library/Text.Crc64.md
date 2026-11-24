# Text.Crc64

Immutable SIMD-built polynomial tables, slicing-by-eight CRC-64, persistent states, network-order digests and table-checked checkpoints.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Crc64</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Crc64/source/">View source</a><a href="/packages/search/?q=%2BText.Crc64">Search module</a><a href="/reference-sources/Text.Crc64.cinch" download="Text.Crc64.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

<details class="doc-notes"><summary>Module notes</summary>

CRC-64 polynomial tables, persistent accumulators and binary checkpoints.
Tables own immutable slicing-by-eight storage; construction generates pairs
of entries with SIMD. Updates combine eight packed input bytes per step and
handle exact tails. Digest and checkpoint words use network order. Retaining
a state branches the computation. This checksum does not authenticate data.

</details>

<details class="doc-synopsis"><summary>Synopsis (25 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Crc64State">Crc64State</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc64State</strong></code></li><li><a class="syn-name" href="#t-Crc64StateError">Crc64StateError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc64StateError</strong></code></li><li><a class="syn-name" href="#t-Crc64Table">Crc64Table</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc64Table</strong></code></li><li><a class="syn-name" href="#v-crc64-ecma">crc64-ecma</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-ecma</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-crc64-iso">crc64-iso</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-iso</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-crc64-size">crc64-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-crc64-state">crc64-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a></code></li><li><a class="syn-name" href="#v-crc64-state-append">crc64-state-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-crc64-state-append-save">crc64-state-append-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-crc64-state-block-size">crc64-state-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-crc64-state-error-message">crc64-state-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64StateError" title="Text.Crc64.Crc64StateError">Crc64StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-crc64-state-load">crc64-state-load</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64StateError" title="Text.Crc64.Crc64StateError">Crc64StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a></code></li><li><a class="syn-name" href="#v-crc64-state-reset">crc64-state-reset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a></code></li><li><a class="syn-name" href="#v-crc64-state-save">crc64-state-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-crc64-state-size">crc64-state-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-crc64-state-sum">crc64-state-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-crc64-state-update">crc64-state-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a></code></li><li><a class="syn-name" href="#v-crc64-state-word">crc64-state-word</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-state-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-crc64-sum">crc64-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-crc64-table">crc64-table</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a></code></li><li><a class="syn-name" href="#v-crc64-table-entries">crc64-table-entries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-table-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-crc64-table-entry">crc64-table-entry</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-table-entry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-crc64-table-sum">crc64-table-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-table-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-crc64-table-update">crc64-table-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-table-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-crc64-update">crc64-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc64-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Crc64State">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc64State</strong></code><span class="doc-sig-links"><a href="#t-Crc64State" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L156" title="View source at line 156">Source</a></span></div>
<div class="doc-body">

An immutable checksum accumulator retaining its immutable prepared table.

</div>
</article>

<article class="doc-entry" id="t-Crc64StateError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc64StateError</strong></code><span class="doc-sig-links"><a href="#t-Crc64StateError" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L211" title="View source at line 211">Source</a></span></div>
<div class="doc-body">

A checkpoint's identifier, exact byte size or prepared table does not match.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidCrc64StateIdentifier"><code><strong class="tok-constructor">InvalidCrc64StateIdentifier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64StateError" title="Text.Crc64.Crc64StateError">Crc64StateError</a></code></li><li id="k-InvalidCrc64StateSize"><code><strong class="tok-constructor">InvalidCrc64StateSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64StateError" title="Text.Crc64.Crc64StateError">Crc64StateError</a></code></li><li id="k-Crc64StateTableMismatch"><code><strong class="tok-constructor">Crc64StateTableMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64StateError" title="Text.Crc64.Crc64StateError">Crc64StateError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Crc64Table">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc64Table</strong></code><span class="doc-sig-links"><a href="#t-Crc64Table" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

An immutable polynomial table with eight derived slices and a checkpoint fingerprint.
Retain and reuse a prepared table across updates to amortize its construction.

</div>
</article>

# Values

<article class="doc-entry" id="v-crc64-ecma">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-ecma</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-ecma" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L249" title="View source at line 249">Source</a></span></div>
<div class="doc-body">

The reflected ECMA CRC-64 polynomial, 0xC96C5795D7870F42.

</div>
</article>

<article class="doc-entry" id="v-crc64-iso">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-iso</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-iso" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L245" title="View source at line 245">Source</a></span></div>
<div class="doc-body">

The reflected ISO CRC-64 polynomial, 0xD800000000000000.

</div>
</article>

<article class="doc-entry" id="v-crc64-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-crc64-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

Digest size in bytes, always eight.

</div>
</article>

<article class="doc-entry" id="v-crc64-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a></code><span class="doc-sig-links"><a href="#v-crc64-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

Start an accumulator at zero with the supplied prepared table.

</div>
</article>

<article class="doc-entry" id="v-crc64-state-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-crc64-state-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L196" title="View source at line 196">Source</a></span></div>
<div class="doc-body">

Append the network-order digest to a copied prefix without changing either argument.

</div>
</article>

<article class="doc-entry" id="v-crc64-state-append-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-crc64-state-append-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L207" title="View source at line 207">Source</a></span></div>
<div class="doc-body">

Append a complete checkpoint to a copied prefix, preserving the state.

</div>
</article>

<article class="doc-entry" id="v-crc64-state-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-crc64-state-block-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

Input block size in bytes, always one; updates accept any chunk length.

</div>
</article>

<article class="doc-entry" id="v-crc64-state-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64StateError" title="Text.Crc64.Crc64StateError">Crc64StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-crc64-state-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L220" title="View source at line 220">Source</a></span></div>
<div class="doc-body">

Render a checkpoint diagnostic, including complete expected and actual byte lengths.

</div>
</article>

<article class="doc-entry" id="v-crc64-state-load">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64StateError" title="Text.Crc64.Crc64StateError">Crc64StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a></code><span class="doc-sig-links"><a href="#v-crc64-state-load" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L228" title="View source at line 228">Source</a></span></div>
<div class="doc-body">

Restore a checkpoint against the supplied immutable table without retaining input bytes.
Identifier checks precede exact-size checks, which precede table matching. Every checksum word is accepted.

</div>
</article>

<article class="doc-entry" id="v-crc64-state-reset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a></code><span class="doc-sig-links"><a href="#v-crc64-state-reset" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Return a zero accumulator retaining the same prepared polynomial table.

</div>
</article>

<article class="doc-entry" id="v-crc64-state-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-crc64-state-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

Return a twenty-byte checkpoint: ASCII crc, version 2, table fingerprint and checksum.
Both words use network order. The fingerprint identifies the complete base table.

</div>
</article>

<article class="doc-entry" id="v-crc64-state-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-crc64-state-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L173" title="View source at line 173">Source</a></span></div>
<div class="doc-body">

Digest size in bytes, always eight.

</div>
</article>

<article class="doc-entry" id="v-crc64-state-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-crc64-state-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L185" title="View source at line 185">Source</a></span></div>
<div class="doc-body">

Return eight fresh digest bytes in network order without changing the state.

</div>
</article>

<article class="doc-entry" id="v-crc64-state-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a></code><span class="doc-sig-links"><a href="#v-crc64-state-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Consume a chunk without changing any earlier state, input buffer or table.

</div>
</article>

<article class="doc-entry" id="v-crc64-state-word">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-state-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64State" title="Text.Crc64.Crc64State">Crc64State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-state-word" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

Observe the complete numeric checksum without advancing or resetting.

</div>
</article>

<article class="doc-entry" id="v-crc64-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L253" title="View source at line 253">Source</a></span></div>
<div class="doc-body">

Compute CRC-64 with the supplied reflected polynomial. The empty checksum is zero.

</div>
</article>

<article class="doc-entry" id="v-crc64-table">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a></code><span class="doc-sig-links"><a href="#v-crc64-table" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Prepare all 256 polynomial entries and seven derived slices in fresh owned storage.
The argument is a reflected polynomial; every U64 value is supported.

</div>
</article>

<article class="doc-entry" id="v-crc64-table-entries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-table-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-table-entries" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

Return all 256 base entries in index order without exposing the owned backing bytes.

</div>
</article>

<article class="doc-entry" id="v-crc64-table-entry">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-table-entry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-table-entry" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Observe a base entry at an index in 0..255; other indices return None.

</div>
</article>

<article class="doc-entry" id="v-crc64-table-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-table-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-table-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

Checksum all bytes from the initial zero state with a reusable prepared table.

</div>
</article>

<article class="doc-entry" id="v-crc64-table-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-table-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc64/#t-Crc64Table" title="Text.Crc64.Crc64Table">Crc64Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-table-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

Continue any finished checksum using a prepared polynomial table.
Empty input preserves every supplied bit; inputs and table storage are never modified.

</div>
</article>

<article class="doc-entry" id="v-crc64-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc64-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-crc64-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc64/source/#L259" title="View source at line 259">Source</a></span></div>
<div class="doc-body">

Continue a finished CRC-64 with a reflected polynomial, preserving empty updates.
Chunks of at least 16384 bytes prepare a slicing table; shorter chunks use
the direct shift register. Reuse crc64-table-update to amortize table setup.

</div>
</article>
