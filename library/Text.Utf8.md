# Text.Utf8

Raw UTF-8 and immutable-text decoding, validation, scalar counts, replacement encoding and checked atomic buffer writes.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Utf8</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Utf8/source/">View source</a><a href="/packages/search/?q=%2BText.Utf8">Search module</a><a href="/reference-sources/Text.Utf8.cinch" download="Text.Utf8.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Char](Data.Char.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Raw UTF-8 byte utilities and constant-space observations of valid immutable text.
Byte APIs preserve malformed input and report exact one-byte replacement progress.
String APIs rely on the valid UTF-8 invariant; their first/last observations
retain exact scalar width without copying the full input or exposing its backing.

</details>

<details class="doc-synopsis"><summary>Synopsis (35 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Utf8Status">Utf8Status</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Utf8Status</strong></code></li><li><a class="syn-name" href="#t-Utf8Step">Utf8Step</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Utf8Step</strong></code></li><li><a class="syn-name" href="#v-utf8-append-rune">utf8-append-rune</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-append-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-utf8-count">utf8-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-count-between">utf8-count-between</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-count-between</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-count-text">utf8-count-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-count-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-decode-at">utf8-decode-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-decode-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code></li><li><a class="syn-name" href="#v-utf8-decode-between">utf8-decode-between</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-decode-between</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code></li><li><a class="syn-name" href="#v-utf8-encode-into">utf8-encode-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-encode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-encode-rune">utf8-encode-rune</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-encode-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-utf8-end-step">utf8-end-step</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-end-step</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code></li><li><a class="syn-name" href="#v-utf8-first">utf8-first</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-first</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code></li><li><a class="syn-name" href="#v-utf8-first-invalid">utf8-first-invalid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-first-invalid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-first-text">utf8-first-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-first-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code></li><li><a class="syn-name" href="#v-utf8-full-rune">utf8-full-rune</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-full-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-utf8-full-rune-text">utf8-full-rune-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-full-rune-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-utf8-last">utf8-last</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-last</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code></li><li><a class="syn-name" href="#v-utf8-last-before">utf8-last-before</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-last-before</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code></li><li><a class="syn-name" href="#v-utf8-last-text">utf8-last-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-last-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code></li><li><a class="syn-name" href="#v-utf8-max-rune">utf8-max-rune</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-max-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-utf8-max-width">utf8-max-width</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-max-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-repair">utf8-repair</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-repair</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-utf8-repair-each">utf8-repair-each</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-repair-each</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-utf8-replace-invalid">utf8-replace-invalid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-replace-invalid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-utf8-rune-error">utf8-rune-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-rune-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-utf8-rune-length">utf8-rune-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-rune-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-rune-self">utf8-rune-self</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-rune-self</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-utf8-rune-start">utf8-rune-start</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-rune-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-utf8-scalar-step">utf8-scalar-step</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-scalar-step</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code></li><li><a class="syn-name" href="#v-utf8-step-char">utf8-step-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-step-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-utf8-step-status">utf8-step-status</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-step-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Status" title="Data.Bytes.Utf8.Utf8Status">Utf8Status</a></code></li><li><a class="syn-name" href="#v-utf8-step-width">utf8-step-width</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-step-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-valid">utf8-valid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-valid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-utf8-valid-rune">utf8-valid-rune</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-valid-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-utf8-valid-text">utf8-valid-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-valid-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Utf8Status">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Utf8Status</strong></code><span class="doc-sig-links"><a href="#t-Utf8Status" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

Whether a decoding position is valid, malformed, exactly at the end, or beyond the buffer.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Utf8End"><code><strong class="tok-constructor">Utf8End</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Status" title="Data.Bytes.Utf8.Utf8Status">Utf8Status</a></code></li><li id="k-Utf8Valid"><code><strong class="tok-constructor">Utf8Valid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Status" title="Data.Bytes.Utf8.Utf8Status">Utf8Status</a></code></li><li id="k-Utf8Invalid"><code><strong class="tok-constructor">Utf8Invalid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Status" title="Data.Bytes.Utf8.Utf8Status">Utf8Status</a></code></li><li id="k-Utf8OutOfBounds"><code><strong class="tok-constructor">Utf8OutOfBounds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Status" title="Data.Bytes.Utf8.Utf8Status">Utf8Status</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Utf8Step">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Utf8Step</strong></code><span class="doc-sig-links"><a href="#t-Utf8Step" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

A decoded scalar, exact consumed byte width and explicit position status.

</div>
</article>

# Values

<article class="doc-entry" id="v-utf8-append-rune">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-append-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-utf8-append-rune" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L501" title="View source at line 501">Source</a></span></div>
<div class="doc-body">

Append one encoded numeric rune to a fresh copy of arbitrary source bytes.

</div>
</article>

<article class="doc-entry" id="v-utf8-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-count" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L421" title="View source at line 421">Source</a></span></div>
<div class="doc-body">

Count scalars with exact one-byte progress on all malformed input and 16-byte SIMD ASCII blocks.

</div>
</article>

<article class="doc-entry" id="v-utf8-count-between">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-count-between</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-count-between" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L431" title="View source at line 431">Source</a></span></div>
<div class="doc-body">

Count decoded scalars in the half-open byte span [start,end), without copying.
Bounds are absolute owner offsets. Return None if start > end or end exceeds
buffer length; valid empty spans return Some 0. Each malformed or truncated
byte counts as one replacement scalar, including a continuation at start.
SIMD/SWAR ASCII blocks and exact scalar fringes never read outside the span.

</div>
</article>

<article class="doc-entry" id="v-utf8-count-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-count-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-count-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf8/source/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

Count scalars using the sealed-valid text counter without copying its UTF-8 backing.

</div>
</article>

<article class="doc-entry" id="v-utf8-decode-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-decode-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code><span class="doc-sig-links"><a href="#v-utf8-decode-at" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L250" title="View source at line 250">Source</a></span></div>
<div class="doc-body">

Decode at a byte offset; a malformed nonempty suffix always consumes exactly one byte.

</div>
</article>

<article class="doc-entry" id="v-utf8-decode-between">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-decode-between</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code><span class="doc-sig-links"><a href="#v-utf8-decode-between" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L262" title="View source at line 262">Source</a></span></div>
<div class="doc-body">

Decode at an absolute owner offset within the half-open byte span [start,end).
Invalid bounds (start > end or end > buffer length), positions before start,
and positions beyond end return Utf8OutOfBounds with zero width and U+FFFD.
A valid at == end returns Utf8End, including an empty span. The span is decoded
independently: a leading continuation or encoding truncated by end consumes
exactly one byte as Utf8Invalid. No byte outside the span is read or copied.

</div>
</article>

<article class="doc-entry" id="v-utf8-encode-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-encode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-encode-into" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L491" title="View source at line 491">Source</a></span></div>
<div class="doc-body">

Encode into an existing buffer, returning written width; insufficient capacity or an invalid offset writes nothing.

</div>
</article>

<article class="doc-entry" id="v-utf8-encode-rune">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-encode-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-utf8-encode-rune" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L496" title="View source at line 496">Source</a></span></div>
<div class="doc-body">

Encode a numeric rune into fresh bytes, replacing invalid scalar inputs with U+FFFD.

</div>
</article>

<article class="doc-entry" id="v-utf8-end-step">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-end-step</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code><span class="doc-sig-links"><a href="#v-utf8-end-step" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L575" title="View source at line 575">Source</a></span></div>
<div class="doc-body">

Construct the empty-suffix observation without allocating or exposing a byte buffer.

</div>
</article>

<article class="doc-entry" id="v-utf8-first">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-first</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code><span class="doc-sig-links"><a href="#v-utf8-first" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L270" title="View source at line 270">Source</a></span></div>
<div class="doc-body">

Decode the first scalar or one malformed byte; an empty buffer has Utf8End status.

</div>
</article>

<article class="doc-entry" id="v-utf8-first-invalid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-first-invalid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-first-invalid" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L358" title="View source at line 358">Source</a></span></div>
<div class="doc-body">

Locate the first malformed byte, preserving end as None and never copying input.

</div>
</article>

<article class="doc-entry" id="v-utf8-first-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-first-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code><span class="doc-sig-links"><a href="#v-utf8-first-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf8/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

Observe the first scalar and exact encoded width; empty text returns Utf8End with zero width.

</div>
</article>

<article class="doc-entry" id="v-utf8-full-rune">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-full-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-utf8-full-rune" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L440" title="View source at line 440">Source</a></span></div>
<div class="doc-body">

Whether a nonempty prefix already determines a decoding step, valid or malformed.
A potentially valid but incomplete sequence is false; an already invalid prefix is true.

</div>
</article>

<article class="doc-entry" id="v-utf8-full-rune-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-full-rune-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-utf8-full-rune-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf8/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Whether valid immutable text contains a complete first scalar; this is exactly nonemptiness.

</div>
</article>

<article class="doc-entry" id="v-utf8-last">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-last</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code><span class="doc-sig-links"><a href="#v-utf8-last" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L301" title="View source at line 301">Source</a></span></div>
<div class="doc-body">

Decode the final scalar or one final malformed byte; empty input has Utf8End status.

</div>
</article>

<article class="doc-entry" id="v-utf8-last-before">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-last-before</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code><span class="doc-sig-links"><a href="#v-utf8-last-before" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L287" title="View source at line 287">Source</a></span></div>
<div class="doc-body">

Decode immediately before a byte offset; malformed endings consume exactly the final byte.

</div>
</article>

<article class="doc-entry" id="v-utf8-last-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-last-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code><span class="doc-sig-links"><a href="#v-utf8-last-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf8/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

Observe the last scalar with a bounded reverse scan; empty text returns Utf8End with zero width.

</div>
</article>

<article class="doc-entry" id="v-utf8-max-rune">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-max-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-utf8-max-rune" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L64" title="View source at line 64">Source</a></span></div>
<div class="doc-body">

The greatest Unicode code point, which is a valid scalar.

</div>
</article>

<article class="doc-entry" id="v-utf8-max-width">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-max-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-max-width" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">

Maximum bytes in a scalar's shortest UTF-8 encoding.

</div>
</article>

<article class="doc-entry" id="v-utf8-repair">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-repair</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-utf8-repair" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L556" title="View source at line 556">Source</a></span></div>
<div class="doc-body">

Replace each maximal run of malformed bytes with one arbitrary byte replacement; output is always fresh.

</div>
</article>

<article class="doc-entry" id="v-utf8-repair-each">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-repair-each</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-utf8-repair-each" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L560" title="View source at line 560">Source</a></span></div>
<div class="doc-body">

Replace every malformed byte separately, retaining exact one-byte decoding progress; output is always fresh.

</div>
</article>

<article class="doc-entry" id="v-utf8-replace-invalid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-replace-invalid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-utf8-replace-invalid" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L564" title="View source at line 564">Source</a></span></div>
<div class="doc-body">

Repair every malformed byte as U+FFFD, producing fresh valid UTF-8 bytes.

</div>
</article>

<article class="doc-entry" id="v-utf8-rune-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-rune-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-utf8-rune-error" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">

The scalar emitted for a malformed byte or invalid numeric encoding input.

</div>
</article>

<article class="doc-entry" id="v-utf8-rune-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-rune-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-rune-length" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L452" title="View source at line 452">Source</a></span></div>
<div class="doc-body">

Return the exact encoded byte width, or None for an invalid numeric rune.

</div>
</article>

<article class="doc-entry" id="v-utf8-rune-self">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-rune-self</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-utf8-rune-self" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

The first byte requiring a multibyte encoding.

</div>
</article>

<article class="doc-entry" id="v-utf8-rune-start">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-rune-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-utf8-rune-start" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L274" title="View source at line 274">Source</a></span></div>
<div class="doc-body">

Test whether a byte can start an encoding, including malformed leading bytes.

</div>
</article>

<article class="doc-entry" id="v-utf8-scalar-step">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-scalar-step</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code><span class="doc-sig-links"><a href="#v-utf8-scalar-step" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L569" title="View source at line 569">Source</a></span></div>
<div class="doc-body">

Construct a valid decoding observation for one scalar, deriving its unique encoded width.
Callers cannot provide inconsistent status or width metadata.

</div>
</article>

<article class="doc-entry" id="v-utf8-step-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-step-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-utf8-step-char" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Observe the decoded scalar, or U+FFFD for an invalid/end/out-of-bounds step.

</div>
</article>

<article class="doc-entry" id="v-utf8-step-status">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-step-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Status" title="Data.Bytes.Utf8.Utf8Status">Utf8Status</a></code><span class="doc-sig-links"><a href="#v-utf8-step-status" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

Observe whether the position represents valid input, a malformed byte, end, or an invalid offset.

</div>
</article>

<article class="doc-entry" id="v-utf8-step-width">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-step-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-step-width" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

Observe consumed bytes: one through four for valid input, one for malformed input, zero at end or beyond.

</div>
</article>

<article class="doc-entry" id="v-utf8-valid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-valid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-utf8-valid" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L365" title="View source at line 365">Source</a></span></div>
<div class="doc-body">

Whether every byte participates in a shortest-form scalar encoding; empty input is valid.

</div>
</article>

<article class="doc-entry" id="v-utf8-valid-rune">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-valid-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-utf8-valid-rune" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Utf8/source/#L448" title="View source at line 448">Source</a></span></div>
<div class="doc-body">

Whether a numeric rune is a Unicode scalar; negatives, surrogates and values above U+10FFFF are false.

</div>
</article>

<article class="doc-entry" id="v-utf8-valid-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-valid-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-utf8-valid-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf8/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

Every value of String is valid UTF-8; malformed arbitrary input is represented by Bytes.

</div>
</article>
