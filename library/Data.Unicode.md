# Data.Unicode

Versioned Unicode classification, simple case mappings, fold cycles, named tables and checked custom ranges.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Unicode</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.Unicode/">View source</a><a href="/library/search/?q=%2BData.Unicode">Search module</a><a href="/reference-sources/Data.Unicode.cinch" download="Data.Unicode.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Names](Data.Unicode.Names.md), [Data.Unicode.Range](Data.Unicode.Range.md).

<details class="doc-closure"><summary>Ship-source closure (6 files)</summary>

Copy these modules beside your entry file before importing `Data.Unicode`: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode](Data.Unicode.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Names](Data.Unicode.Names.md), [Data.Unicode.Range](Data.Unicode.Range.md), [Data.Unicode.Tables](Data.Unicode.Tables.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Data/Unicode.cinch ./Data.Unicode.cinch
cp seed/prelude/Data/Unicode/Kernel.cinch ./Data.Unicode.Kernel.cinch
cp seed/prelude/Data/Unicode/Names.cinch ./Data.Unicode.Names.cinch
cp seed/prelude/Data/Unicode/Range.cinch ./Data.Unicode.Range.cinch
cp seed/prelude/Data/Unicode/Tables.cinch ./Data.Unicode.Tables.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Unicode 17.0.0 classification, simple casing, fold cycles and immutable named range tables.
Character APIs accept only Unicode scalars. Numeric table membership includes
all Unicode code points, including surrogates; UTF-8 validity is a separate
operation. Case/fold numeric helpers retain invalid values, and checked custom
tables report malformed bounds, strides, ordering and metadata explicitly.
Built-in classification never materializes or copies the complete tables.

</details>

<details class="doc-synopsis"><summary>Synopsis (141 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-UnicodeCase">UnicodeCase</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCase</strong></code></li><li><a class="syn-name" href="#t-UnicodeCaseError">UnicodeCaseError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCaseError</strong></code></li><li><a class="syn-name" href="#t-UnicodeCaseRange">UnicodeCaseRange</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCaseRange</strong></code></li><li><a class="syn-name" href="#t-UnicodeRange16">UnicodeRange16</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRange16</strong></code></li><li><a class="syn-name" href="#t-UnicodeRange32">UnicodeRange32</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRange32</strong></code></li><li><a class="syn-name" href="#t-UnicodeRangeError">UnicodeRangeError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRangeError</strong></code></li><li><a class="syn-name" href="#t-UnicodeRangeTable">UnicodeRangeTable</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRangeTable</strong></code></li><li><a class="syn-name" href="#t-UnicodeSpecialCase">UnicodeSpecialCase</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeSpecialCase</strong></code></li><li><a class="syn-name" href="#v-unicode-azeri-case">unicode-azeri-case</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-azeri-case</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code></li><li><a class="syn-name" href="#v-unicode-case-count">unicode-case-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-case-error-index">unicode-case-error-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-error-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-case-error-message">unicode-case-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-case-index">unicode-case-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-at">unicode-case-range-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-count">unicode-case-range-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-high">unicode-case-range-high</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-high</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-low">unicode-case-range-low</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-low</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-lower">unicode-case-range-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-title">unicode-case-range-title</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-title</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-upper">unicode-case-range-upper</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-upper</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-case-ranges">unicode-case-ranges</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code></li><li><a class="syn-name" href="#v-unicode-category">unicode-category</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-category</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-category-alias-at">unicode-category-alias-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-category-alias-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-unicode-category-alias-count">unicode-category-alias-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-category-alias-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-category-aliases">unicode-category-aliases</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-category-aliases</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-unicode-category-canonical">unicode-category-canonical</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-category-canonical</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-category-count">unicode-category-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-category-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-category-name">unicode-category-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-category-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-category-names">unicode-category-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-category-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-category-of-code">unicode-category-of-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-category-of-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-category-resolve">unicode-category-resolve</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-category-resolve</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-equal-fold">unicode-equal-fold</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-equal-fold</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-equal-fold-code">unicode-equal-fold-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-equal-fold-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-fold-category">unicode-fold-category</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-fold-category</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-fold-category-count">unicode-fold-category-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-fold-category-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-fold-category-name">unicode-fold-category-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-fold-category-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-fold-category-names">unicode-fold-category-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-fold-category-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-fold-script">unicode-fold-script</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-fold-script</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-fold-script-count">unicode-fold-script-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-fold-script-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-fold-script-name">unicode-fold-script-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-fold-script-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-fold-script-names">unicode-fold-script-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-fold-script-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-graphic-ranges">unicode-graphic-ranges</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-graphic-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-in">unicode-in</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-in</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-in-code">unicode-in-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-in-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-control">unicode-is-control</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-control</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-control-code">unicode-is-control-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-control-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-digit">unicode-is-digit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-digit</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-digit-code">unicode-is-digit-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-digit-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-graphic">unicode-is-graphic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-graphic-code">unicode-is-graphic-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-graphic-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-letter">unicode-is-letter</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-letter</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-letter-code">unicode-is-letter-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-letter-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-lower">unicode-is-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-lower-code">unicode-is-lower-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-mark">unicode-is-mark</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-mark</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-mark-code">unicode-is-mark-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-mark-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-number">unicode-is-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-number</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-number-code">unicode-is-number-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-number-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-one-of">unicode-is-one-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-one-of</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-one-of-code">unicode-is-one-of-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-one-of-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-print">unicode-is-print</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-print</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-print-code">unicode-is-print-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-print-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-punct">unicode-is-punct</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-punct</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-punct-code">unicode-is-punct-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-punct-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-scalar-code">unicode-is-scalar-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-scalar-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-space">unicode-is-space</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-space</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-space-code">unicode-is-space-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-space-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-symbol">unicode-is-symbol</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-symbol</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-symbol-code">unicode-is-symbol-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-symbol-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-title">unicode-is-title</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-title</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-title-code">unicode-is-title-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-upper">unicode-is-upper</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-upper</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-upper-code">unicode-is-upper-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-latin-offset">unicode-latin-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-latin-offset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-max-ascii">unicode-max-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-max-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-max-codepoint">unicode-max-codepoint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-max-codepoint</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-max-latin1">unicode-max-latin1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-max-latin1</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-max-rune">unicode-max-rune</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-max-rune</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-print-ranges">unicode-print-ranges</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-print-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-property">unicode-property</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-property</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-property-count">unicode-property-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-property-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-property-name">unicode-property-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-property-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-property-names">unicode-property-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-property-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-range-contains">unicode-range-contains</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-contains</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-range-contains-code">unicode-range-contains-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-contains-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-range-error-index">unicode-range-error-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-error-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-range-error-is-wide">unicode-range-error-is-wide</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-error-is-wide</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-range-error-message">unicode-range-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-range-table">unicode-range-table</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-table</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-range-table-clone">unicode-range-table-clone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-table-clone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-range-table-with-offset">unicode-range-table-with-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range-table-with-offset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-range16-at">unicode-range16-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range16-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a></code></li><li><a class="syn-name" href="#v-unicode-range16-count">unicode-range16-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range16-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-range16-high">unicode-range16-high</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range16-high</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-unicode-range16-low">unicode-range16-low</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range16-low</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-unicode-range16-ranges">unicode-range16-ranges</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range16-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a></code></li><li><a class="syn-name" href="#v-unicode-range16-stride">unicode-range16-stride</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range16-stride</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-unicode-range32-at">unicode-range32-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range32-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a></code></li><li><a class="syn-name" href="#v-unicode-range32-count">unicode-range32-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range32-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-range32-high">unicode-range32-high</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range32-high</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-range32-low">unicode-range32-low</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range32-low</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-range32-ranges">unicode-range32-ranges</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range32-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a></code></li><li><a class="syn-name" href="#v-unicode-range32-stride">unicode-range32-stride</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-range32-stride</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-replacement">unicode-replacement</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-replacement</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-script">unicode-script</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-script</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-script-count">unicode-script-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-script-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-script-name">unicode-script-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-script-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-script-names">unicode-script-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-script-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-script-of-code">unicode-script-of-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-script-of-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-simple-fold">unicode-simple-fold</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-simple-fold</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-simple-fold-code">unicode-simple-fold-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-simple-fold-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-special-affects-ascii">unicode-special-affects-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-affects-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-special-case">unicode-special-case</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-case</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code></li><li><a class="syn-name" href="#v-unicode-special-case-count">unicode-special-case-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-case-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-special-case-range-at">unicode-special-case-range-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-case-range-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code></li><li><a class="syn-name" href="#v-unicode-special-case-ranges">unicode-special-case-ranges</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-case-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code></li><li><a class="syn-name" href="#v-unicode-special-to">unicode-special-to</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-lower">unicode-special-to-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-lower-code">unicode-special-to-lower-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-title">unicode-special-to-title</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-title</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-title-code">unicode-special-to-title-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-upper">unicode-special-to-upper</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-upper</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-upper-code">unicode-special-to-upper-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-table">unicode-table</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-table</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code></li><li><a class="syn-name" href="#v-unicode-table-count">unicode-table-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-table-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-table-name">unicode-table-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-table-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-table-names">unicode-table-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-table-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-to">unicode-to</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-to-code">unicode-to-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-to-lower">unicode-to-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-to-lower-code">unicode-to-lower-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-to-title">unicode-to-title</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-title</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-to-title-code">unicode-to-title-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-to-upper">unicode-to-upper</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-upper</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-to-upper-code">unicode-to-upper-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-turkish-case">unicode-turkish-case</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-turkish-case</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code></li><li><a class="syn-name" href="#v-unicode-upper-lower">unicode-upper-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-upper-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-version">unicode-version</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-version</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unicode-version-major">unicode-version-major</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-version-major</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-version-minor">unicode-version-minor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-version-minor</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-version-patch">unicode-version-patch</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-version-patch</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-UnicodeCase">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCase</strong></code><span class="doc-sig-links"><a href="#t-UnicodeCase" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

One of the three simple, single-scalar case mappings.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnicodeUpper"><code><strong class="tok-constructor">UnicodeUpper</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a></code></li><li id="k-UnicodeLower"><code><strong class="tok-constructor">UnicodeLower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a></code></li><li id="k-UnicodeTitle"><code><strong class="tok-constructor">UnicodeTitle</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeCaseError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCaseError</strong></code><span class="doc-sig-links"><a href="#t-UnicodeCaseError" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L370" title="View source at line 370">Source</a></span></div>
<div class="doc-body">

A precise invalid custom-case rule, carrying its zero-based rule index.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnicodeCaseInvalidBounds"><code><strong class="tok-constructor">UnicodeCaseInvalidBounds</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a></code></li><li id="k-UnicodeCaseInvalidOrder"><code><strong class="tok-constructor">UnicodeCaseInvalidOrder</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a></code></li><li id="k-UnicodeCaseInvalidMapping"><code><strong class="tok-constructor">UnicodeCaseInvalidMapping</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a></code></li><li id="k-UnicodeCaseInvalidAlternating"><code><strong class="tok-constructor">UnicodeCaseInvalidAlternating</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a></code></li><li id="k-UnicodeCaseTooManyRules"><code><strong class="tok-constructor">UnicodeCaseTooManyRules</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeCaseRange">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCaseRange</strong></code><span class="doc-sig-links"><a href="#t-UnicodeCaseRange" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L345" title="View source at line 345">Source</a></span></div>
<div class="doc-body">

A complete inclusive simple-case rule with signed upper, lower and title deltas.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkUnicodeCaseRange"><code><strong class="tok-constructor">MkUnicodeCaseRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeRange16">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRange16</strong></code><span class="doc-sig-links"><a href="#t-UnicodeRange16" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

An inclusive 16-bit strided code-point interval, which may include surrogates.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkUnicodeRange16"><code><strong class="tok-constructor">MkUnicodeRange16</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeRange32">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRange32</strong></code><span class="doc-sig-links"><a href="#t-UnicodeRange32" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

An inclusive supplementary-plane strided code-point interval.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkUnicodeRange32"><code><strong class="tok-constructor">MkUnicodeRange32</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeRangeError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRangeError</strong></code><span class="doc-sig-links"><a href="#t-UnicodeRangeError" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

A precise invalid custom interval or metadata observation.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnicodeRangeInvalidBounds"><code><strong class="tok-constructor">UnicodeRangeInvalidBounds</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a></code></li><li id="k-UnicodeRangeZeroStride"><code><strong class="tok-constructor">UnicodeRangeZeroStride</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a></code></li><li id="k-UnicodeRangeInvalidOrder"><code><strong class="tok-constructor">UnicodeRangeInvalidOrder</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a></code></li><li id="k-UnicodeRangeInvalidLatinOffset"><code><strong class="tok-constructor">UnicodeRangeInvalidLatinOffset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a></code></li><li id="k-UnicodeRangeTooManyRecords"><code><strong class="tok-constructor">UnicodeRangeTooManyRecords</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeRangeTable">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeRangeTable</strong></code><span class="doc-sig-links"><a href="#t-UnicodeRangeTable" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

An immutable built-in table or private copied custom interval storage.

</div>
</article>

<article class="doc-entry" id="t-UnicodeSpecialCase">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeSpecialCase</strong></code><span class="doc-sig-links"><a href="#t-UnicodeSpecialCase" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L392" title="View source at line 392">Source</a></span></div>
<div class="doc-body">

An immutable locale override set; explicit identity deltas suppress default mappings.

</div>
</article>

# Values

<article class="doc-entry" id="v-unicode-azeri-case">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-azeri-case</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code><span class="doc-sig-links"><a href="#v-unicode-azeri-case" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L403" title="View source at line 403">Source</a></span></div>
<div class="doc-body">

Azerbaijani simple-case overrides, sharing the immutable Turkic rules.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Number of independent simple-case mapping selectors.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-error-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-error-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-case-error-index" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L383" title="View source at line 383">Source</a></span></div>
<div class="doc-body">

Observe the zero-based index of an invalid custom-case rule.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-case-error-message" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L142" title="View source at line 142">Source</a></span></div>
<div class="doc-body">

Format a precise checked custom-case construction failure with its rule index.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-index" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Observe the stable numeric selector associated with one case.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-at" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L572" title="View source at line 572">Source</a></span></div>
<div class="doc-body">

Observe one standard case rule; out-of-range indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L581" title="View source at line 581">Source</a></span></div>
<div class="doc-body">

Count complete standard case range records without allocating their observations.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-high">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-high</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-high" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L354" title="View source at line 354">Source</a></span></div>
<div class="doc-body">

The upper inclusive source code point of a case rule.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-low">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-low</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-low" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L350" title="View source at line 350">Source</a></span></div>
<div class="doc-body">

The lower inclusive source code point of a case rule.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-lower" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L362" title="View source at line 362">Source</a></span></div>
<div class="doc-body">

The signed lowercase delta, or the alternating-case sentinel.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-title">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-title</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-title" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L366" title="View source at line 366">Source</a></span></div>
<div class="doc-body">

The signed titlecase delta, or the alternating-case sentinel.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-upper">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-upper</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-upper" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L358" title="View source at line 358">Source</a></span></div>
<div class="doc-body">

The signed uppercase delta, or the alternating-case sentinel.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-ranges">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code><span class="doc-sig-links"><a href="#v-unicode-case-ranges" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

Observe every exact immutable standard case range without exposing backing storage.

</div>
</article>

<article class="doc-entry" id="v-unicode-category">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-category</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-category" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L24" title="View source at line 24">Source</a></span></div>
<div class="doc-body">

Resolve one exact immutable unicode-category name without normalizing punctuation or case.

</div>
</article>

<article class="doc-entry" id="v-unicode-category-alias-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-category-alias-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-unicode-category-alias-at" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L3459" title="View source at line 3459">Source</a></span></div>
<div class="doc-body">

Observe one sorted alias and its canonical category name; invalid indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-category-alias-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-category-alias-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-category-alias-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L3455" title="View source at line 3455">Source</a></span></div>
<div class="doc-body">

Number of immutable category-alias observations.

</div>
</article>

<article class="doc-entry" id="v-unicode-category-aliases">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-category-aliases</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-unicode-category-aliases" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L138" title="View source at line 138">Source</a></span></div>
<div class="doc-body">

Observe the complete immutable category-alias map as sorted key/value pairs.

</div>
</article>

<article class="doc-entry" id="v-unicode-category-canonical">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-category-canonical</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-category-canonical" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L3408" title="View source at line 3408">Source</a></span></div>
<div class="doc-body">

Resolve an exact standard category alias to its canonical abbreviated name.

</div>
</article>

<article class="doc-entry" id="v-unicode-category-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-category-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-category-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L219" title="View source at line 219">Source</a></span></div>
<div class="doc-body">

Count the exact unicode-category catalog keys, including documented aliases.

</div>
</article>

<article class="doc-entry" id="v-unicode-category-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-category-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-category-name" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L223" title="View source at line 223">Source</a></span></div>
<div class="doc-body">

Read one sorted immutable unicode-category key; invalid indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-category-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-category-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-category-names" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L283" title="View source at line 283">Source</a></span></div>
<div class="doc-body">

Observe every unicode-category key without exposing or copying range-table storage.

</div>
</article>

<article class="doc-entry" id="v-unicode-category-of-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-category-of-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-category-of-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L3506" title="View source at line 3506">Source</a></span></div>
<div class="doc-body">

Observe the primary category, including Cs and Cn; values outside the Unicode code-point domain return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-category-resolve">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-category-resolve</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-category-resolve" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Resolve a canonical category name first, then an exact standard category alias.

</div>
</article>

<article class="doc-entry" id="v-unicode-equal-fold">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-equal-fold</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-equal-fold" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L210" title="View source at line 210">Source</a></span></div>
<div class="doc-body">

Test whether two scalars belong to the same simple-fold cycle.

</div>
</article>

<article class="doc-entry" id="v-unicode-equal-fold-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-equal-fold-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-equal-fold-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L204" title="View source at line 204">Source</a></span></div>
<div class="doc-body">

Test numeric simple-fold equivalence; identical invalid inputs compare equal.

</div>
</article>

<article class="doc-entry" id="v-unicode-fold-category">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-fold-category</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-fold-category" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1649" title="View source at line 1649">Source</a></span></div>
<div class="doc-body">

Resolve one exact immutable unicode-fold-category name without normalizing punctuation or case.

</div>
</article>

<article class="doc-entry" id="v-unicode-fold-category-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-fold-category-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-fold-category-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1684" title="View source at line 1684">Source</a></span></div>
<div class="doc-body">

Count the exact unicode-fold-category catalog keys, including documented aliases.

</div>
</article>

<article class="doc-entry" id="v-unicode-fold-category-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-fold-category-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-fold-category-name" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1688" title="View source at line 1688">Source</a></span></div>
<div class="doc-body">

Read one sorted immutable unicode-fold-category key; invalid indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-fold-category-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-fold-category-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-fold-category-names" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1716" title="View source at line 1716">Source</a></span></div>
<div class="doc-body">

Observe every unicode-fold-category key without exposing or copying range-table storage.

</div>
</article>

<article class="doc-entry" id="v-unicode-fold-script">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-fold-script</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-fold-script" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1720" title="View source at line 1720">Source</a></span></div>
<div class="doc-body">

Resolve one exact immutable unicode-fold-script name without normalizing punctuation or case.

</div>
</article>

<article class="doc-entry" id="v-unicode-fold-script-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-fold-script-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-fold-script-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1740" title="View source at line 1740">Source</a></span></div>
<div class="doc-body">

Count the exact unicode-fold-script catalog keys, including documented aliases.

</div>
</article>

<article class="doc-entry" id="v-unicode-fold-script-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-fold-script-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-fold-script-name" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1744" title="View source at line 1744">Source</a></span></div>
<div class="doc-body">

Read one sorted immutable unicode-fold-script key; invalid indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-fold-script-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-fold-script-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-fold-script-names" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1769" title="View source at line 1769">Source</a></span></div>
<div class="doc-body">

Observe every unicode-fold-script key without exposing or copying range-table storage.

</div>
</article>

<article class="doc-entry" id="v-unicode-graphic-ranges">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-graphic-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-graphic-ranges" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L122" title="View source at line 122">Source</a></span></div>
<div class="doc-body">

Immutable category tables defining graphic characters: letters, marks, numbers, punctuation, symbols and space separators.

</div>
</article>

<article class="doc-entry" id="v-unicode-in">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-in</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-in" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L306" title="View source at line 306">Source</a></span></div>
<div class="doc-body">

Test scalar membership in any supplied immutable table.

</div>
</article>

<article class="doc-entry" id="v-unicode-in-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-in-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-in-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L302" title="View source at line 302">Source</a></span></div>
<div class="doc-body">

Test numeric membership in any supplied immutable table, retaining invalid-input policy.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-control">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-control</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-control" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L218" title="View source at line 218">Source</a></span></div>
<div class="doc-body">

Test the Unicode control class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-control-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-control-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-control-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L214" title="View source at line 214">Source</a></span></div>
<div class="doc-body">

Test the Unicode control class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-digit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-digit</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-digit" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L226" title="View source at line 226">Source</a></span></div>
<div class="doc-body">

Test the Unicode digit class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-digit-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-digit-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-digit-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L222" title="View source at line 222">Source</a></span></div>
<div class="doc-body">

Test the Unicode digit class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-graphic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-graphic" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L298" title="View source at line 298">Source</a></span></div>
<div class="doc-body">

Test the Unicode graphic class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-graphic-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-graphic-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-graphic-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L294" title="View source at line 294">Source</a></span></div>
<div class="doc-body">

Test the Unicode graphic class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-letter">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-letter</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-letter" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L234" title="View source at line 234">Source</a></span></div>
<div class="doc-body">

Test the Unicode letter class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-letter-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-letter-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-letter-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L230" title="View source at line 230">Source</a></span></div>
<div class="doc-body">

Test the Unicode letter class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-lower" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L242" title="View source at line 242">Source</a></span></div>
<div class="doc-body">

Test the Unicode lower class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-lower-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-lower-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L238" title="View source at line 238">Source</a></span></div>
<div class="doc-body">

Test the Unicode lower class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-mark">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-mark</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-mark" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L266" title="View source at line 266">Source</a></span></div>
<div class="doc-body">

Test the Unicode mark class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-mark-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-mark-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-mark-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L262" title="View source at line 262">Source</a></span></div>
<div class="doc-body">

Test the Unicode mark class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-number</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-number" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L274" title="View source at line 274">Source</a></span></div>
<div class="doc-body">

Test the Unicode number class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-number-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-number-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-number-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L270" title="View source at line 270">Source</a></span></div>
<div class="doc-body">

Test the Unicode number class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-one-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-one-of</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-one-of" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L314" title="View source at line 314">Source</a></span></div>
<div class="doc-body">

Test a table list before a scalar, with the same union semantics as unicode-in.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-one-of-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-one-of-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-one-of-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L310" title="View source at line 310">Source</a></span></div>
<div class="doc-body">

Test a table list before a numeric code point, with the same union semantics as unicode-in-code.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-print">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-print</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-print" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L306" title="View source at line 306">Source</a></span></div>
<div class="doc-body">

Test the Unicode print class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-print-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-print-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-print-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L302" title="View source at line 302">Source</a></span></div>
<div class="doc-body">

Test the Unicode print class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-punct">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-punct</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-punct" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L282" title="View source at line 282">Source</a></span></div>
<div class="doc-body">

Test the Unicode punct class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-punct-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-punct-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-punct-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L278" title="View source at line 278">Source</a></span></div>
<div class="doc-body">

Test the Unicode punct class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-scalar-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-scalar-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-scalar-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

Test a signed numeric value for Unicode scalar validity, excluding surrogates.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-space">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-space</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-space" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L317" title="View source at line 317">Source</a></span></div>
<div class="doc-body">

Test whether one scalar is Unicode whitespace.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-space-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-space-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-space-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L310" title="View source at line 310">Source</a></span></div>
<div class="doc-body">

Test Unicode whitespace, including Latin-1 separators and the full White_Space property.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-symbol">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-symbol</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-symbol" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L290" title="View source at line 290">Source</a></span></div>
<div class="doc-body">

Test the Unicode symbol class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-symbol-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-symbol-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-symbol-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L286" title="View source at line 286">Source</a></span></div>
<div class="doc-body">

Test the Unicode symbol class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-title">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-title</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-title" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L258" title="View source at line 258">Source</a></span></div>
<div class="doc-body">

Test the Unicode title class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-title-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-title-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L254" title="View source at line 254">Source</a></span></div>
<div class="doc-body">

Test the Unicode title class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-upper">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-upper</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-upper" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L250" title="View source at line 250">Source</a></span></div>
<div class="doc-body">

Test the Unicode upper class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-upper-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-upper-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L246" title="View source at line 246">Source</a></span></div>
<div class="doc-body">

Test the Unicode upper class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-latin-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-latin-offset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-latin-offset" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

Observe the exact original built-in Latin offset, or the calculated custom prefix count.

</div>
</article>

<article class="doc-entry" id="v-unicode-max-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-max-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-max-ascii" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

Highest ASCII code point.

</div>
</article>

<article class="doc-entry" id="v-unicode-max-codepoint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-max-codepoint</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-max-codepoint" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

Highest Unicode code point; this bound alone does not exclude surrogates.

</div>
</article>

<article class="doc-entry" id="v-unicode-max-latin1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-max-latin1</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-max-latin1" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

Highest Latin-1 code point.

</div>
</article>

<article class="doc-entry" id="v-unicode-max-rune">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-max-rune</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-max-rune" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

The greatest valid Unicode scalar value, including its noncharacter status.

</div>
</article>

<article class="doc-entry" id="v-unicode-print-ranges">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-print-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-print-ranges" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

Immutable category tables used by print classification; the separate ASCII-space rule remains observable.

</div>
</article>

<article class="doc-entry" id="v-unicode-property">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-property</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-property" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1380" title="View source at line 1380">Source</a></span></div>
<div class="doc-body">

Resolve one exact immutable unicode-property name without normalizing punctuation or case.

</div>
</article>

<article class="doc-entry" id="v-unicode-property-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-property-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-property-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1580" title="View source at line 1580">Source</a></span></div>
<div class="doc-body">

Count the exact unicode-property catalog keys, including documented aliases.

</div>
</article>

<article class="doc-entry" id="v-unicode-property-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-property-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-property-name" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1584" title="View source at line 1584">Source</a></span></div>
<div class="doc-body">

Read one sorted immutable unicode-property key; invalid indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-property-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-property-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-property-names" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1645" title="View source at line 1645">Source</a></span></div>
<div class="doc-body">

Observe every unicode-property key without exposing or copying range-table storage.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-contains">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-contains</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-range-contains" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L298" title="View source at line 298">Source</a></span></div>
<div class="doc-body">

Test the membership of one valid scalar without copying or exposing a backing table.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-contains-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-contains-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-range-contains-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L287" title="View source at line 287">Source</a></span></div>
<div class="doc-body">

Test numeric membership, including surrogates and excluding negative or out-of-domain values.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-error-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-error-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-range-error-index" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Observe the zero-based invalid interval index; metadata-only failures return zero.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-error-is-wide">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-error-is-wide</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-range-error-is-wide" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Identify an invalid interval in the supplementary-plane half; metadata-only failures return false.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-range-error-message" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L148" title="View source at line 148">Source</a></span></div>
<div class="doc-body">

Format a precise checked custom-range construction failure without reading invalid backing storage.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-table">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-table</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-range-table" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

Construct a validated custom code-point table, including legitimate surrogate intervals.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-table-clone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-table-clone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-range-table-clone" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

Retain an independent immutable table value without copying private storage.

</div>
</article>

<article class="doc-entry" id="v-unicode-range-table-with-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range-table-with-offset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeError" title="Data.Unicode.Range.UnicodeRangeError">UnicodeRangeError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-range-table-with-offset" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

Construct a custom table only when the explicit Latin prefix matches its validated metadata.

</div>
</article>

<article class="doc-entry" id="v-unicode-range16-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range16-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a></code><span class="doc-sig-links"><a href="#v-unicode-range16-at" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L215" title="View source at line 215">Source</a></span></div>
<div class="doc-body">

Observe a 16-bit record by zero-based index; invalid indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-range16-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range16-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-range16-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

Observe the original number of 16-bit records without materializing the table.

</div>
</article>

<article class="doc-entry" id="v-unicode-range16-high">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range16-high</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-unicode-range16-high" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

The upper inclusive 16-bit bound, whether or not the stride lands on it.

</div>
</article>

<article class="doc-entry" id="v-unicode-range16-low">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range16-low</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-unicode-range16-low" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

The lower inclusive 16-bit bound.

</div>
</article>

<article class="doc-entry" id="v-unicode-range16-ranges">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range16-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a></code><span class="doc-sig-links"><a href="#v-unicode-range16-ranges" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

Observe every exact 16-bit range record in its original order.

</div>
</article>

<article class="doc-entry" id="v-unicode-range16-stride">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range16-stride</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange16" title="Data.Unicode.Range.UnicodeRange16">UnicodeRange16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-unicode-range16-stride" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

The exact original 16-bit stride.

</div>
</article>

<article class="doc-entry" id="v-unicode-range32-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range32-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a></code><span class="doc-sig-links"><a href="#v-unicode-range32-at" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L222" title="View source at line 222">Source</a></span></div>
<div class="doc-body">

Observe a supplementary-plane record by zero-based index; invalid indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-range32-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range32-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-range32-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

Observe the original number of supplementary-plane records without materializing them.

</div>
</article>

<article class="doc-entry" id="v-unicode-range32-high">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range32-high</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-range32-high" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

The upper inclusive supplementary-plane bound, whether or not the stride lands on it.

</div>
</article>

<article class="doc-entry" id="v-unicode-range32-low">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range32-low</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-range32-low" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

The lower inclusive supplementary-plane bound.

</div>
</article>

<article class="doc-entry" id="v-unicode-range32-ranges">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range32-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a></code><span class="doc-sig-links"><a href="#v-unicode-range32-ranges" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

Observe every exact supplementary-plane range record in its original order.

</div>
</article>

<article class="doc-entry" id="v-unicode-range32-stride">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-range32-stride</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRange32" title="Data.Unicode.Range.UnicodeRange32">UnicodeRange32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-range32-stride" title="Permalink">#</a><a href="/library/source/Data.Unicode.Range/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

The exact original 32-bit stride, including values larger than the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-replacement">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-replacement</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-replacement" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

The Unicode replacement scalar U+FFFD.

</div>
</article>

<article class="doc-entry" id="v-unicode-script">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-script</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-script" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L287" title="View source at line 287">Source</a></span></div>
<div class="doc-body">

Resolve one exact immutable unicode-script name without normalizing punctuation or case.

</div>
</article>

<article class="doc-entry" id="v-unicode-script-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-script-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-script-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1162" title="View source at line 1162">Source</a></span></div>
<div class="doc-body">

Count the exact unicode-script catalog keys, including documented aliases.

</div>
</article>

<article class="doc-entry" id="v-unicode-script-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-script-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-script-name" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1166" title="View source at line 1166">Source</a></span></div>
<div class="doc-body">

Read one sorted immutable unicode-script key; invalid indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-script-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-script-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-script-names" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1376" title="View source at line 1376">Source</a></span></div>
<div class="doc-body">

Observe every unicode-script key without exposing or copying range-table storage.

</div>
</article>

<article class="doc-entry" id="v-unicode-script-of-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-script-of-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-script-of-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L3555" title="View source at line 3555">Source</a></span></div>
<div class="doc-body">

Observe the primary script; values outside the code-point domain or without a named script return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-simple-fold">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-simple-fold</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-simple-fold" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L194" title="View source at line 194">Source</a></span></div>
<div class="doc-body">

Return the next scalar in its simple-fold cycle without normalization or expansion.

</div>
</article>

<article class="doc-entry" id="v-unicode-simple-fold-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-simple-fold-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-simple-fold-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L184" title="View source at line 184">Source</a></span></div>
<div class="doc-body">

Return the numeric-order successor in a simple-fold cycle; invalid numbers remain unchanged.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-affects-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-affects-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-special-affects-ascii" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L609" title="View source at line 609">Source</a></span></div>
<div class="doc-body">

Whether any override can affect ASCII, allowing bulk default ASCII mapping when false.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-case">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-case</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code><span class="doc-sig-links"><a href="#v-unicode-special-case" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L451" title="View source at line 451">Source</a></span></div>
<div class="doc-body">

Construct scalar-preserving ordered custom rules, with identity rules overriding default casing.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-case-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-case-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-special-case-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L553" title="View source at line 553">Source</a></span></div>
<div class="doc-body">

Count immutable override records without materializing their observations.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-case-range-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-case-range-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code><span class="doc-sig-links"><a href="#v-unicode-special-case-range-at" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L566" title="View source at line 566">Source</a></span></div>
<div class="doc-body">

Observe one override record; out-of-range indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-case-ranges">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-case-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code><span class="doc-sig-links"><a href="#v-unicode-special-case-ranges" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

Observe every immutable override rule without exposing its private byte storage.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L549" title="View source at line 549">Source</a></span></div>
<div class="doc-body">

Apply one scalar-preserving locale override mapping to a valid character.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-lower" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L597" title="View source at line 597">Source</a></span></div>
<div class="doc-body">

Apply scalar-preserving locale lowercase overrides before default casing.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-lower-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-lower-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L593" title="View source at line 593">Source</a></span></div>
<div class="doc-body">

Apply locale lowercase overrides to a numeric code point, preserving invalid inputs.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-title">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-title</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-title" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L605" title="View source at line 605">Source</a></span></div>
<div class="doc-body">

Apply scalar-preserving locale titlecase overrides before default casing.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-title-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-title-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L601" title="View source at line 601">Source</a></span></div>
<div class="doc-body">

Apply locale titlecase overrides to a numeric code point, preserving invalid inputs.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-upper">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-upper</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-upper" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L589" title="View source at line 589">Source</a></span></div>
<div class="doc-body">

Apply scalar-preserving locale uppercase overrides before default casing.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-upper-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-upper-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L585" title="View source at line 585">Source</a></span></div>
<div class="doc-body">

Apply locale uppercase overrides to a numeric code point, preserving invalid inputs.

</div>
</article>

<article class="doc-entry" id="v-unicode-table">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-table</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeRangeTable" title="Data.Unicode.Range.UnicodeRangeTable">UnicodeRangeTable</a></code><span class="doc-sig-links"><a href="#v-unicode-table" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L1773" title="View source at line 1773">Source</a></span></div>
<div class="doc-body">

Resolve one exact immutable unicode-table name without normalizing punctuation or case.

</div>
</article>

<article class="doc-entry" id="v-unicode-table-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-table-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-table-count" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L3088" title="View source at line 3088">Source</a></span></div>
<div class="doc-body">

Count the exact unicode-table catalog keys, including documented aliases.

</div>
</article>

<article class="doc-entry" id="v-unicode-table-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-table-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-table-name" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L3092" title="View source at line 3092">Source</a></span></div>
<div class="doc-body">

Read one sorted immutable unicode-table key; invalid indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-table-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-table-names</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-table-names" title="Permalink">#</a><a href="/library/source/Data.Unicode.Names/#L3404" title="View source at line 3404">Source</a></span></div>
<div class="doc-body">

Observe every unicode-table key without exposing or copying range-table storage.

</div>
</article>

<article class="doc-entry" id="v-unicode-to">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-to" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Apply one simple case mapping to a valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-to-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Map a signed selector and numeric code point; an invalid selector returns U+FFFD.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-to-lower" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L333" title="View source at line 333">Source</a></span></div>
<div class="doc-body">

Return the simple single-scalar lowercase mapping.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-lower-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-to-lower-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L329" title="View source at line 329">Source</a></span></div>
<div class="doc-body">

Return the simple lowercase mapping of a numeric code point; invalid inputs remain unchanged.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-title">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-title</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-to-title" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L341" title="View source at line 341">Source</a></span></div>
<div class="doc-body">

Return the simple single-scalar titlecase mapping.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-title-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-to-title-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L337" title="View source at line 337">Source</a></span></div>
<div class="doc-body">

Return the simple titlecase mapping of a numeric code point; invalid inputs remain unchanged.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-upper">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-upper</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-to-upper" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L325" title="View source at line 325">Source</a></span></div>
<div class="doc-body">

Return the simple single-scalar uppercase mapping.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-upper-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-to-upper-code" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L321" title="View source at line 321">Source</a></span></div>
<div class="doc-body">

Return the simple uppercase mapping of a numeric code point; invalid inputs remain unchanged.

</div>
</article>

<article class="doc-entry" id="v-unicode-turkish-case">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-turkish-case</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code><span class="doc-sig-links"><a href="#v-unicode-turkish-case" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L399" title="View source at line 399">Source</a></span></div>
<div class="doc-body">

Turkish simple-case overrides, taking precedence over default mappings.

</div>
</article>

<article class="doc-entry" id="v-unicode-upper-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-upper-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-upper-lower" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Sentinel delta used by alternating uppercase/lowercase range observations.

</div>
</article>

<article class="doc-entry" id="v-unicode-version">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-version</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unicode-version" title="Permalink">#</a><a href="/library/source/Data.Unicode/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

Exact edition of every built-in classification, script, property and simple mapping.

</div>
</article>

<article class="doc-entry" id="v-unicode-version-major">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-version-major</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-version-major" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Major edition of the immutable classification and mapping data.

</div>
</article>

<article class="doc-entry" id="v-unicode-version-minor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-version-minor</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-version-minor" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

Minor edition of the immutable classification and mapping data.

</div>
</article>

<article class="doc-entry" id="v-unicode-version-patch">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-version-patch</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-version-patch" title="Permalink">#</a><a href="/library/source/Data.Unicode.Kernel/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Patch edition of the immutable classification and mapping data.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Unicode.cinch`](/library/source/Data.Unicode/). Source SHA-256: `fba58333635e0995c1ae77eb1947275c7198a557147a95333f8ef52d981da2c0`.
