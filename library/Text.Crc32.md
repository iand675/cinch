# Text.Crc32

Prepared reflected polynomial tables, persistent CRC-32 states and table-checked binary checkpoints.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Crc32</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Crc32/source/">View source</a><a href="/packages/search/?q=%2BText.Crc32">Search module</a><a href="/reference-sources/Text.Crc32.cinch" download="Text.Crc32.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

<details class="doc-notes"><summary>Module notes</summary>

Prepared reflected CRC-32 tables, immutable accumulators and portable binary checkpoints.

</details>

<details class="doc-synopsis"><summary>Synopsis (21 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Crc32State">Crc32State</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc32State</strong></code></li><li><a class="syn-name" href="#t-Crc32StateError">Crc32StateError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc32StateError</strong></code></li><li><a class="syn-name" href="#t-Crc32Table">Crc32Table</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc32Table</strong></code></li><li><a class="syn-name" href="#v-crc32-size">crc32-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-crc32-state">crc32-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a></code></li><li><a class="syn-name" href="#v-crc32-state-append">crc32-state-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-crc32-state-append-save">crc32-state-append-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-crc32-state-block-size">crc32-state-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-crc32-state-error-message">crc32-state-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32StateError" title="Text.Crc32.Crc32StateError">Crc32StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-crc32-state-load">crc32-state-load</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32StateError" title="Text.Crc32.Crc32StateError">Crc32StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a></code></li><li><a class="syn-name" href="#v-crc32-state-reset">crc32-state-reset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a></code></li><li><a class="syn-name" href="#v-crc32-state-save">crc32-state-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-crc32-state-size">crc32-state-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-crc32-state-sum">crc32-state-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-crc32-state-update">crc32-state-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a></code></li><li><a class="syn-name" href="#v-crc32-state-word">crc32-state-word</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-state-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc32-table">crc32-table</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a></code></li><li><a class="syn-name" href="#v-crc32-table-entries">crc32-table-entries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-table-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc32-table-entry">crc32-table-entry</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-table-entry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc32-table-sum">crc32-table-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-table-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc32-table-update">crc32-table-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-table-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Crc32State">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc32State</strong></code><span class="doc-sig-links"><a href="#t-Crc32State" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

An immutable checksum accumulator retaining its immutable prepared table.

</div>
</article>

<article class="doc-entry" id="t-Crc32StateError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc32StateError</strong></code><span class="doc-sig-links"><a href="#t-Crc32StateError" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

A checkpoint's identifier, exact byte size or prepared table does not match.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidCrc32StateIdentifier"><code><strong class="tok-constructor">InvalidCrc32StateIdentifier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32StateError" title="Text.Crc32.Crc32StateError">Crc32StateError</a></code></li><li id="k-InvalidCrc32StateSize"><code><strong class="tok-constructor">InvalidCrc32StateSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32StateError" title="Text.Crc32.Crc32StateError">Crc32StateError</a></code></li><li id="k-Crc32StateTableMismatch"><code><strong class="tok-constructor">Crc32StateTableMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32StateError" title="Text.Crc32.Crc32StateError">Crc32StateError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Crc32Table">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Crc32Table</strong></code><span class="doc-sig-links"><a href="#t-Crc32Table" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

Private little-endian base entries and their IEEE fingerprint.

</div>
</article>

# Values

<article class="doc-entry" id="v-crc32-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-crc32-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

Number of bytes in a CRC-32 digest.

</div>
</article>

<article class="doc-entry" id="v-crc32-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a></code><span class="doc-sig-links"><a href="#v-crc32-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

Start an accumulator at zero with the supplied prepared table.

</div>
</article>

<article class="doc-entry" id="v-crc32-state-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-crc32-state-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L150" title="View source at line 150">Source</a></span></div>
<div class="doc-body">

Append the network-order digest to a copied prefix without changing either argument.

</div>
</article>

<article class="doc-entry" id="v-crc32-state-append-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-crc32-state-append-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

Append a complete checkpoint to a copied prefix, preserving the state.

</div>
</article>

<article class="doc-entry" id="v-crc32-state-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-crc32-state-block-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

Input block size in bytes, always one; updates accept any chunk length.

</div>
</article>

<article class="doc-entry" id="v-crc32-state-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32StateError" title="Text.Crc32.Crc32StateError">Crc32StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-crc32-state-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L174" title="View source at line 174">Source</a></span></div>
<div class="doc-body">

Render a checkpoint diagnostic, including complete expected and actual byte lengths.

</div>
</article>

<article class="doc-entry" id="v-crc32-state-load">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32StateError" title="Text.Crc32.Crc32StateError">Crc32StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a></code><span class="doc-sig-links"><a href="#v-crc32-state-load" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L182" title="View source at line 182">Source</a></span></div>
<div class="doc-body">

Restore a checkpoint against the supplied immutable table without retaining input bytes.
Identifier checks precede exact-size checks, which precede table matching. Every checksum word is accepted.

</div>
</article>

<article class="doc-entry" id="v-crc32-state-reset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a></code><span class="doc-sig-links"><a href="#v-crc32-state-reset" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

Return a zero accumulator retaining the same prepared polynomial table.

</div>
</article>

<article class="doc-entry" id="v-crc32-state-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-crc32-state-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L155" title="View source at line 155">Source</a></span></div>
<div class="doc-body">

Return a twelve-byte checkpoint: ASCII crc, version 1, table fingerprint and checksum.
Both words use network order. The fingerprint identifies the complete base table.

</div>
</article>

<article class="doc-entry" id="v-crc32-state-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-crc32-state-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

Digest size in bytes, always four.

</div>
</article>

<article class="doc-entry" id="v-crc32-state-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-crc32-state-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L139" title="View source at line 139">Source</a></span></div>
<div class="doc-body">

Return four fresh digest bytes in network order without changing the state.

</div>
</article>

<article class="doc-entry" id="v-crc32-state-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a></code><span class="doc-sig-links"><a href="#v-crc32-state-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

Consume a chunk without changing any earlier state, input buffer or table.

</div>
</article>

<article class="doc-entry" id="v-crc32-state-word">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-state-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32-state-word" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

Observe the complete numeric checksum without advancing or resetting.

</div>
</article>

<article class="doc-entry" id="v-crc32-table">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a></code><span class="doc-sig-links"><a href="#v-crc32-table" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

Prepare all 256 entries for any reflected polynomial.

</div>
</article>

<article class="doc-entry" id="v-crc32-table-entries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-table-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32-table-entries" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Return all 256 base entries in index order without exposing the owned backing bytes.

</div>
</article>

<article class="doc-entry" id="v-crc32-table-entry">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-table-entry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32-table-entry" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L71" title="View source at line 71">Source</a></span></div>
<div class="doc-body">

Observe a base entry at an index in 0..255; other indices return None.

</div>
</article>

<article class="doc-entry" id="v-crc32-table-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-table-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32-table-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

Checksum all bytes from the initial zero state with a reusable prepared table.

</div>
</article>

<article class="doc-entry" id="v-crc32-table-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-table-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Crc32/#t-Crc32Table" title="Text.Crc32.Crc32Table">Crc32Table</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32-table-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Crc32/source/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

Continue any finished checksum using a prepared polynomial table.
Empty input preserves every supplied bit; inputs and table storage are never modified.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Crc32.cinch`](/packages/stdlib/Text.Crc32/source/). Source SHA-256: `f12c03f0ef5f001f56a823a066de69aaea06dea088b89059822d5ce1ed9d5aaa`.
