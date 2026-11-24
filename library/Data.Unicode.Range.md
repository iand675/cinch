# Data.Unicode.Range

Immutable Unicode range tables with checked construction, exact membership and copied observations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Unicode.Range</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Unicode.Range/source/">View source</a><a href="/packages/search/?q=%2BData.Unicode.Range">Search module</a><a href="/reference-sources/Data.Unicode.Range.cinch" download="Data.Unicode.Range.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.Unicode.Tables](Data.Unicode.Tables.md).

<details class="doc-notes"><summary>Module notes</summary>

Immutable strided Unicode range tables with exact indexed observations.
Numeric membership includes surrogate code points; scalar construction and
UTF-8 validity remain separate operations. Built-in queries read the compact
class database without copying a table. Custom tables validate ordered bounds,
nonzero strides and Latin-1 metadata before creating private owned storage.

</details>

<details class="doc-synopsis"><summary>Synopsis (27 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-UnicodeRange16">UnicodeRange16</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRange16</strong></code></li><li><a class="syn-name" href="#t-UnicodeRange32">UnicodeRange32</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRange32</strong></code></li><li><a class="syn-name" href="#t-UnicodeRangeError">UnicodeRangeError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRangeError</strong></code></li><li><a class="syn-name" href="#t-UnicodeRangeTable">UnicodeRangeTable</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRangeTable</strong></code></li><li><a class="syn-name" href="#v-unicode-builtin-range-table">unicode-builtin-range-table</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-builtin-range-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-in">unicode-in</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-in-code">unicode-in-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-in-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-one-of">unicode-is-one-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-one-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-one-of-code">unicode-is-one-of-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-one-of-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-latin-offset">unicode-latin-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-latin-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-range-contains">unicode-range-contains</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-contains</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-range-contains-code">unicode-range-contains-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-contains-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-range-error-index">unicode-range-error-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-error-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-range-error-is-wide">unicode-range-error-is-wide</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-error-is-wide</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-range-table">unicode-range-table</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-range-table-clone">unicode-range-table-clone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-table-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-range-table-with-offset">unicode-range-table-with-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-table-with-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-range16-at">unicode-range16-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range16-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a></code></li><li><a class="syn-name" href="#v-unicode-range16-count">unicode-range16-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range16-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-range16-high">unicode-range16-high</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range16-high</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-unicode-range16-low">unicode-range16-low</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range16-low</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-unicode-range16-stride">unicode-range16-stride</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range16-stride</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-unicode-range32-at">unicode-range32-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range32-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a></code></li><li><a class="syn-name" href="#v-unicode-range32-count">unicode-range32-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range32-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-range32-high">unicode-range32-high</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range32-high</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-range32-low">unicode-range32-low</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range32-low</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-range32-stride">unicode-range32-stride</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range32-stride</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-UnicodeRange16">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRange16</strong></code><span class="doc-sig-links"><a href="#t-UnicodeRange16" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

An inclusive 16-bit strided code-point interval, which may include surrogates.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkUnicodeRange16"><code><strong class="tok-constructor">MkUnicodeRange16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeRange32">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRange32</strong></code><span class="doc-sig-links"><a href="#t-UnicodeRange32" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

An inclusive supplementary-plane strided code-point interval.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkUnicodeRange32"><code><strong class="tok-constructor">MkUnicodeRange32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeRangeError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRangeError</strong></code><span class="doc-sig-links"><a href="#t-UnicodeRangeError" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

A precise invalid custom interval or metadata observation.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnicodeRangeInvalidBounds"><code><strong class="tok-constructor">UnicodeRangeInvalidBounds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a></code></li><li id="k-UnicodeRangeZeroStride"><code><strong class="tok-constructor">UnicodeRangeZeroStride</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a></code></li><li id="k-UnicodeRangeInvalidOrder"><code><strong class="tok-constructor">UnicodeRangeInvalidOrder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a></code></li><li id="k-UnicodeRangeInvalidLatinOffset"><code><strong class="tok-constructor">UnicodeRangeInvalidLatinOffset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a></code></li><li id="k-UnicodeRangeTooManyRecords"><code><strong class="tok-constructor">UnicodeRangeTooManyRecords</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeRangeTable">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRangeTable</strong></code><span class="doc-sig-links"><a href="#t-UnicodeRangeTable" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

An immutable built-in table or private copied custom interval storage.

</div>
</article>

# Values

<article class="doc-entry" id="v-unicode-builtin-range-table">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-builtin-range-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-builtin-range-table" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

Select one immutable built-in identity, rejecting out-of-range internal identifiers.

</div>
</article>

<article class="doc-entry" id="v-unicode-in">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-in" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L306" title="View source at line 306">Source</a></span></div>
<div class="doc-body">

Test scalar membership in any supplied immutable table.

</div>
</article>

<article class="doc-entry" id="v-unicode-in-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-in-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-in-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L302" title="View source at line 302">Source</a></span></div>
<div class="doc-body">

Test numeric membership in any supplied immutable table, retaining invalid-input policy.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-one-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-one-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-one-of" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L314" title="View source at line 314">Source</a></span></div>
<div class="doc-body">

Test a table list before a scalar, with the same union semantics as unicode-in.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-one-of-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-one-of-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-one-of-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L310" title="View source at line 310">Source</a></span></div>
<div class="doc-body">

Test a table list before a numeric code point, with the same union semantics as unicode-in-code.

</div>
</article>

<article class="doc-entry" id="v-unicode-latin-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-latin-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-latin-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

Observe the exact original built-in Latin offset, or the calculated custom prefix count.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-contains">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-contains</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-range-contains" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L298" title="View source at line 298">Source</a></span></div>
<div class="doc-body">

Test the membership of one valid scalar without copying or exposing a backing table.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-contains-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-contains-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-range-contains-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L287" title="View source at line 287">Source</a></span></div>
<div class="doc-body">

Test numeric membership, including surrogates and excluding negative or out-of-domain values.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-error-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-error-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-range-error-index" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Observe the zero-based invalid interval index; metadata-only failures return zero.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-error-is-wide">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-error-is-wide</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-range-error-is-wide" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Identify an invalid interval in the supplementary-plane half; metadata-only failures return false.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-table">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-range-table" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

Construct a validated custom code-point table, including legitimate surrogate intervals.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-table-clone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-table-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-range-table-clone" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

Retain an independent immutable table value without copying private storage.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-table-with-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-table-with-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-range-table-with-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

Construct a custom table only when the explicit Latin prefix matches its validated metadata.

</div>
</article>

<article class="doc-entry" id="v-unicode-range16-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range16-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a></code><span class="doc-sig-links"><a href="#v-unicode-range16-at" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L215" title="View source at line 215">Source</a></span></div>
<div class="doc-body">

Observe a 16-bit record by zero-based index; invalid indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-range16-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range16-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-range16-count" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

Observe the original number of 16-bit records without materializing the table.

</div>
</article>

<article class="doc-entry" id="v-unicode-range16-high">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range16-high</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-unicode-range16-high" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

The upper inclusive 16-bit bound, whether or not the stride lands on it.

</div>
</article>

<article class="doc-entry" id="v-unicode-range16-low">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range16-low</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-unicode-range16-low" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

The lower inclusive 16-bit bound.

</div>
</article>

<article class="doc-entry" id="v-unicode-range16-stride">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range16-stride</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-unicode-range16-stride" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

The exact original 16-bit stride.

</div>
</article>

<article class="doc-entry" id="v-unicode-range32-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range32-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a></code><span class="doc-sig-links"><a href="#v-unicode-range32-at" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L222" title="View source at line 222">Source</a></span></div>
<div class="doc-body">

Observe a supplementary-plane record by zero-based index; invalid indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-range32-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range32-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-range32-count" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

Observe the original number of supplementary-plane records without materializing them.

</div>
</article>

<article class="doc-entry" id="v-unicode-range32-high">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range32-high</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-range32-high" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

The upper inclusive supplementary-plane bound, whether or not the stride lands on it.

</div>
</article>

<article class="doc-entry" id="v-unicode-range32-low">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range32-low</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-range32-low" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

The lower inclusive supplementary-plane bound.

</div>
</article>

<article class="doc-entry" id="v-unicode-range32-stride">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range32-stride</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-range32-stride" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Range/source/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

The exact original 32-bit stride, including values larger than the Unicode domain.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Unicode/Range.cinch`](/packages/stdlib/Data.Unicode.Range/source/). Source SHA-256: `36f645bc334d75474d5dd70f1aa680d330a97476a89a41ee872d880620c036c8`.
