# Text.TabWriter

Elastic tabstops: align tab-separated cells into padded columns, with escapes, HTML-aware widths, right alignment and incremental flushing.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.TabWriter</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.TabWriter/source/">View source</a><a href="/packages/search/?q=%2BText.TabWriter">Search module</a><a href="/reference-sources/Text.TabWriter.cinch" download="Text.TabWriter.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md).

<details class="doc-notes"><summary>Module notes</summary>

Elastic tabstops: align tab-separated cells into columns, the way terminal tables and aligned listings need.
Input is a sequence of cells, each terminated by a horizontal tab ('\t') or
a vertical tab ('\v'); a newline ends a line and a form feed ('\f') ends a
line and forces output. Consecutive lines whose cells reach the same column
form a column block; every cell in a block is padded to the block's widest
cell plus the configured padding. The last cell of each line is never
aligned. Text between two 0xff escape bytes is copied literally (its tabs and
newlines do not split cells), and with tab-filter-html, HTML tags have zero
width and entities a width of one. Cell width counts Unicode scalars, using
SWAR to skip eight bytes at a time; in malformed UTF-8 every byte that is not
a continuation byte counts as one column.

The writer is an immutable value: tab-write returns the output that became
final (blocks are released at a form feed or after a line with a single
cell) together with the advanced writer, and tab-flush releases everything
buffered. Retaining a writer branches the formatting.

</details>

<details class="doc-synopsis" open><summary>Synopsis (14 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-TabConfig">TabConfig</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TabConfig</strong></code></li><li><a class="syn-name" href="#t-TabWriter">TabWriter</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TabWriter</strong></code></li><li><a class="syn-name" href="#v-tab-align-right">tab-align-right</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-align-right</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tab-config">tab-config</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-config</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 255 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabConfig" title="Text.TabWriter.TabConfig">TabConfig</a></code></li><li><a class="syn-name" href="#v-tab-debug">tab-debug</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-debug</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tab-discard-empty-columns">tab-discard-empty-columns</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-discard-empty-columns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tab-escape">tab-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tab-filter-html">tab-filter-html</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-filter-html</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tab-flush">tab-flush</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-flush</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabWriter" title="Text.TabWriter.TabWriter">TabWriter</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabWriter" title="Text.TabWriter.TabWriter">TabWriter</a>)</code></li><li><a class="syn-name" href="#v-tab-format">tab-format</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-format</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabConfig" title="Text.TabWriter.TabConfig">TabConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tab-indent">tab-indent</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-indent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tab-strip-escape">tab-strip-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-strip-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tab-write">tab-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabWriter" title="Text.TabWriter.TabWriter">TabWriter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabWriter" title="Text.TabWriter.TabWriter">TabWriter</a>)</code></li><li><a class="syn-name" href="#v-tab-writer">tab-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tab-writer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabConfig" title="Text.TabWriter.TabConfig">TabConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabWriter" title="Text.TabWriter.TabWriter">TabWriter</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-TabConfig">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TabConfig</strong></code><span class="doc-sig-links"><a href="#t-TabConfig" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Formatting parameters: minimum cell width, tab width (for tab padding), padding added to the
widest cell, the padding byte, and the bitwise union of the flag constants.

</div>
</article>

<article class="doc-entry" id="t-TabWriter">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TabWriter</strong></code><span class="doc-sig-links"><a href="#t-TabWriter" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L94" title="View source at line 94">Source</a></span></div>
<div class="doc-body">

Buffered lines (most recent first, each line's cells in order), the current line's finished
cells (most recent first), the current cell's chunks (most recent first) and width, and the lexer.

</div>
</article>

# Values

<article class="doc-entry" id="v-tab-align-right">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-align-right</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tab-align-right" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Pad cells on the left, aligning their text to the right edge of the column.

</div>
</article>

<article class="doc-entry" id="v-tab-config">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-config</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 255 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabConfig" title="Text.TabWriter.TabConfig">TabConfig</a></code><span class="doc-sig-links"><a href="#v-tab-config" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Build a configuration. A padding byte of 9 (tab) pads with tabs of the given tab width, rounding
column widths up to multiples of it; any other byte pads with repetitions of that byte.
Tab padding implies left alignment, so tab-align-right is cleared when the padding byte is a tab.

</div>
</article>

<article class="doc-entry" id="v-tab-debug">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-debug</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tab-debug" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Print a vertical bar between columns and a dashed line after each form feed.

</div>
</article>

<article class="doc-entry" id="v-tab-discard-empty-columns">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-discard-empty-columns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tab-discard-empty-columns" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

Give columns whose cells are all empty and soft-terminated a width of zero.

</div>
</article>

<article class="doc-entry" id="v-tab-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tab-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

The escape byte, 0xff, that brackets literal text.

</div>
</article>

<article class="doc-entry" id="v-tab-filter-html">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-filter-html</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tab-filter-html" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

Treat HTML tags as zero width and entities as width one; both are copied unchanged.

</div>
</article>

<article class="doc-entry" id="v-tab-flush">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-flush</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabWriter" title="Text.TabWriter.TabWriter">TabWriter</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabWriter" title="Text.TabWriter.TabWriter">TabWriter</a>)</code><span class="doc-sig-links"><a href="#v-tab-flush" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

Release every buffered line and the pending cell, ending any open escape, as final output.
The final line receives no newline of its own; the writer is empty afterwards.

</div>
</article>

<article class="doc-entry" id="v-tab-format">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-format</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabConfig" title="Text.TabWriter.TabConfig">TabConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tab-format" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

Format an entire input as one write followed by a flush.

</div>
</article>

<article class="doc-entry" id="v-tab-indent">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-indent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tab-indent" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

Pad leading empty cells with tabs, whatever the padding byte, so indentation stays tab-based.

</div>
</article>

<article class="doc-entry" id="v-tab-strip-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-strip-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tab-strip-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

Remove the 0xff escape bytes that bracket literal text instead of copying them.

</div>
</article>

<article class="doc-entry" id="v-tab-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabWriter" title="Text.TabWriter.TabWriter">TabWriter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabWriter" title="Text.TabWriter.TabWriter">TabWriter</a>)</code><span class="doc-sig-links"><a href="#v-tab-write" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Consume input and return the output that became final, with the advanced writer.

</div>
</article>

<article class="doc-entry" id="v-tab-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tab-writer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabConfig" title="Text.TabWriter.TabConfig">TabConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TabWriter/#t-TabWriter" title="Text.TabWriter.TabWriter">TabWriter</a></code><span class="doc-sig-links"><a href="#v-tab-writer" title="Permalink">#</a><a href="/packages/stdlib/Text.TabWriter/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

A writer with nothing buffered.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/TabWriter.cinch`](/packages/stdlib/Text.TabWriter/source/). Source SHA-256: `e13da1b41483f29493fe929ca885b7a91a5483f3a3b11ff56482fc034161acc5`.
