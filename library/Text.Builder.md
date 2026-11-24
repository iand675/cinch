# Text.Builder

An efficient text-construction vocabulary for assembling many fragments before rendering.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Builder</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Builder/">View source</a><a href="/library/search/?q=%2BText.Builder">Search module</a><a href="/reference-sources/Text.Builder.cinch" download="Text.Builder.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Text.Builder`: [Text.Builder](Text.Builder.md).

```sh
cp seed/prelude/Text/Builder.cinch ./Text.Builder.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Text.Builder: an append-cheap text and byte builder (the codegen rung's
accumulator).

The ladder's IR rung renders LLVM IR as TEXT with `string-concat`, which
copies its LEFT operand, so emitting a module piece by piece is quadratic in
the number of pieces: survivable for a toy function, hostile for the whole
prelude's 55 modules, and the same shape would bite the manifest writer and
every diagnostic renderer. This module is the efficient accumulator the text
homes did not have.

The representation is a SPINE of pending pieces (the `Bootstrap.Ast`
`*-spine` convention, D387's answer to a data type that cannot nest a `List`
of itself): each append allocates one cell and RETAINS its argument, so an
append of a string or a byte slice copies NOTHING and runs in constant time,
independent of the piece's length. The spine holds the pieces most-recent
first; the flatten lays them out in APPEND order by filling the output
backward from its end. `builder-run`/`builder-bytes` measure the whole spine
once (`builder-size`), allocate ONE output buffer, and copy each piece
exactly once: O(total bytes) time and one allocation, never quadratic.

`builder-size` is EXACT (the sum of each piece's byte length, computed
without flattening) and O(number of pieces). `builder-chunks` counts the
pending pieces, one per append (an append of a line is still one piece), so
a caller can assert that appends do not re-copy.

`builder-add-nat`/`builder-add-int` render through `Text.Show`'s
`nat-to-string`/`int-to-string` and append the resulting `String` as one
piece. `builder-add-line` appends the text and a single LF as one piece, so
the line semantics are the builder's, not a caller's.

`builder-run` decodes the flattened UTF-8. A builder built only through the
string, line, nat and int surface is always valid UTF-8, so the decode never
fails there; a raw byte chunk that is not valid UTF-8 (only
`builder-add-bytes` can insert one) still flattens exactly through
`builder-bytes`, and `builder-run` then yields the empty string. That is the
defined total fallback, the same shape `Text.Show`'s `wrap-bytes` uses.

Module scheme (design/base-library.md): `Text.*` is conversion to and from
String/Bytes, so the builder lives here.

</details>

<details class="doc-synopsis" open><summary>Synopsis (12 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Builder">Builder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Builder</strong></code></li><li><a class="syn-name" href="#v-builder-add-bytes">builder-add-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">builder-add-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code></li><li><a class="syn-name" href="#v-builder-add-int">builder-add-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">builder-add-int</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code></li><li><a class="syn-name" href="#v-builder-add-line">builder-add-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">builder-add-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code></li><li><a class="syn-name" href="#v-builder-add-nat">builder-add-nat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">builder-add-nat</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code></li><li><a class="syn-name" href="#v-builder-add-string">builder-add-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">builder-add-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code></li><li><a class="syn-name" href="#v-builder-bytes">builder-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">builder-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-builder-chunks">builder-chunks</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">builder-chunks</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-builder-is-empty">builder-is-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">builder-is-empty</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-builder-new">builder-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">builder-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code></li><li><a class="syn-name" href="#v-builder-run">builder-run</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">builder-run</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-builder-size">builder-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">builder-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Builder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Builder</strong></code><span class="doc-sig-links"><a href="#t-Builder" title="Permalink">#</a><a href="/library/source/Text.Builder/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

A pending piece, each a retained argument with no copy made at append
time. `BEnd` is the spine's end marker; `BLine` carries the text before
its newline; `BBytes` carries a raw byte slice (which need not be UTF-8).

</div>
</article>

# Values

<article class="doc-entry" id="v-builder-add-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">builder-add-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code><span class="doc-sig-links"><a href="#v-builder-add-bytes" title="Permalink">#</a><a href="/library/source/Text.Builder/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Append a raw byte slice as one piece: O(1), no copy of the slice.

</div>
</article>

<article class="doc-entry" id="v-builder-add-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">builder-add-int</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code><span class="doc-sig-links"><a href="#v-builder-add-int" title="Permalink">#</a><a href="/library/source/Text.Builder/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

Append a decimal `Int` (rendered by `Text.Show`).

</div>
</article>

<article class="doc-entry" id="v-builder-add-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">builder-add-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code><span class="doc-sig-links"><a href="#v-builder-add-line" title="Permalink">#</a><a href="/library/source/Text.Builder/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Append the text and a single LF as one piece.

</div>
</article>

<article class="doc-entry" id="v-builder-add-nat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">builder-add-nat</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code><span class="doc-sig-links"><a href="#v-builder-add-nat" title="Permalink">#</a><a href="/library/source/Text.Builder/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

Append a decimal `Nat` (rendered by `Text.Show`).

</div>
</article>

<article class="doc-entry" id="v-builder-add-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">builder-add-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code><span class="doc-sig-links"><a href="#v-builder-add-string" title="Permalink">#</a><a href="/library/source/Text.Builder/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Append a string as one piece: O(1), no copy of the string's bytes.

</div>
</article>

<article class="doc-entry" id="v-builder-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">builder-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-builder-bytes" title="Permalink">#</a><a href="/library/source/Text.Builder/#L184" title="View source at line 184">Source</a></span></div>
<div class="doc-body">

Exact size and fixed-length pieces make the checked copy pass succeed.
Preserve the original semantics if those private invariants ever disagree:
recompute with the range-checked fill, never return truncated text.

</div>
</article>

<article class="doc-entry" id="v-builder-chunks">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">builder-chunks</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-builder-chunks" title="Permalink">#</a><a href="/library/source/Text.Builder/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

How many pieces are pending: one per append, so a test can assert that
appends do not re-copy. O(number of pieces).

</div>
</article>

<article class="doc-entry" id="v-builder-is-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">builder-is-empty</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-builder-is-empty" title="Permalink">#</a><a href="/library/source/Text.Builder/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Whether a builder has no pending pieces. Structural: a builder is empty
exactly when it was never appended to, so an appended empty string counts
as a piece.

</div>
</article>

<article class="doc-entry" id="v-builder-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">builder-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a></code><span class="doc-sig-links"><a href="#v-builder-new" title="Permalink">#</a><a href="/library/source/Text.Builder/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

The empty builder (no pending pieces).

</div>
</article>

<article class="doc-entry" id="v-builder-run">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">builder-run</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-builder-run" title="Permalink">#</a><a href="/library/source/Text.Builder/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

The flattened string: flatten once, then decode the UTF-8. A builder
built through the string, line, nat and int surface is always valid UTF-8;
a raw byte chunk that is not yields the empty string (see the header).

</div>
</article>

<article class="doc-entry" id="v-builder-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">builder-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Builder/#t-Builder" title="Text.Builder.Builder">Builder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-builder-size" title="Permalink">#</a><a href="/library/source/Text.Builder/#L106" title="View source at line 106">Source</a></span></div>
<div class="doc-body">

The exact total byte length of the flattened result, without flattening
(a line piece counts its LF). O(number of pieces).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Builder.cinch`](/library/source/Text.Builder/). Source SHA-256: `c089e433293b637862387c71967e800c5cb57f4f7011724ee4d21b18c5ece24b`.
