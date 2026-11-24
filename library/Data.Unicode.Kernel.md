# Data.Unicode.Kernel

Unicode scalar and numeric classification, simple case mappings, fold cycles and checked locale overrides.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Unicode.Kernel</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Unicode.Kernel/source/">View source</a><a href="/packages/search/?q=%2BData.Unicode.Kernel">Search module</a><a href="/reference-sources/Data.Unicode.Kernel.cinch" download="Data.Unicode.Kernel.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.Unicode.Tables](Data.Unicode.Tables.md).

<details class="doc-notes"><summary>Module notes</summary>

Unicode 17.0.0 scalar classification, simple casing and simple-fold cycles.
This numeric core has no direct String dependency. Character APIs operate on
scalars; signed code-point APIs retain invalid numeric casing/folding inputs.
Classification rejects out-of-domain numbers while permitting surrogate
code points in numeric table observations. Case conversion returns one scalar;
simple folding walks canonical numeric-order equivalence cycles.

</details>

<details class="doc-synopsis"><summary>Synopsis (74 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-UnicodeCase">UnicodeCase</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCase</strong></code></li><li><a class="syn-name" href="#t-UnicodeCaseError">UnicodeCaseError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCaseError</strong></code></li><li><a class="syn-name" href="#t-UnicodeCaseRange">UnicodeCaseRange</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCaseRange</strong></code></li><li><a class="syn-name" href="#t-UnicodeSpecialCase">UnicodeSpecialCase</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeSpecialCase</strong></code></li><li><a class="syn-name" href="#v-unicode-azeri-case">unicode-azeri-case</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-azeri-case</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code></li><li><a class="syn-name" href="#v-unicode-case-count">unicode-case-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-case-error-index">unicode-case-error-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-error-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-case-index">unicode-case-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-at">unicode-case-range-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-count">unicode-case-range-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-high">unicode-case-range-high</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-high</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-low">unicode-case-range-low</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-low</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-lower">unicode-case-range-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-title">unicode-case-range-title</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-title</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-case-range-upper">unicode-case-range-upper</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-upper</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-equal-fold">unicode-equal-fold</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-equal-fold</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-equal-fold-code">unicode-equal-fold-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-equal-fold-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-control">unicode-is-control</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-control</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-control-code">unicode-is-control-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-control-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-digit">unicode-is-digit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-digit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-digit-code">unicode-is-digit-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-digit-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-graphic">unicode-is-graphic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-graphic-code">unicode-is-graphic-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-graphic-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-letter">unicode-is-letter</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-letter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-letter-code">unicode-is-letter-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-letter-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-lower">unicode-is-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-lower-code">unicode-is-lower-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-mark">unicode-is-mark</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-mark</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-mark-code">unicode-is-mark-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-mark-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-number">unicode-is-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-number-code">unicode-is-number-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-number-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-print">unicode-is-print</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-print</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-print-code">unicode-is-print-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-print-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-punct">unicode-is-punct</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-punct</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-punct-code">unicode-is-punct-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-punct-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-scalar-code">unicode-is-scalar-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-scalar-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-space">unicode-is-space</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-space</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-space-code">unicode-is-space-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-space-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-symbol">unicode-is-symbol</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-symbol</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-symbol-code">unicode-is-symbol-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-symbol-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-title">unicode-is-title</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-title</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-title-code">unicode-is-title-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-upper">unicode-is-upper</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-upper</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-is-upper-code">unicode-is-upper-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-is-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-max-ascii">unicode-max-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-max-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-max-codepoint">unicode-max-codepoint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-max-codepoint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-max-latin1">unicode-max-latin1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-max-latin1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-replacement">unicode-replacement</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-replacement</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-simple-fold">unicode-simple-fold</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-simple-fold</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-simple-fold-code">unicode-simple-fold-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-simple-fold-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-special-affects-ascii">unicode-special-affects-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-affects-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unicode-special-case">unicode-special-case</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-case</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code></li><li><a class="syn-name" href="#v-unicode-special-case-count">unicode-special-case-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-case-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-unicode-special-case-range-at">unicode-special-case-range-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-case-range-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code></li><li><a class="syn-name" href="#v-unicode-special-to">unicode-special-to</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-lower">unicode-special-to-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-lower-code">unicode-special-to-lower-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-title">unicode-special-to-title</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-title</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-title-code">unicode-special-to-title-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-upper">unicode-special-to-upper</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-upper</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-special-to-upper-code">unicode-special-to-upper-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-to">unicode-to</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-to-code">unicode-to-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-to-lower">unicode-to-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-to-lower-code">unicode-to-lower-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-to-title">unicode-to-title</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-title</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-to-title-code">unicode-to-title-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-to-upper">unicode-to-upper</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-upper</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-unicode-to-upper-code">unicode-to-upper-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-to-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-turkish-case">unicode-turkish-case</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-turkish-case</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code></li><li><a class="syn-name" href="#v-unicode-upper-lower">unicode-upper-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-upper-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-unicode-version-major">unicode-version-major</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-version-major</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-version-minor">unicode-version-minor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-version-minor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-unicode-version-patch">unicode-version-patch</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unicode-version-patch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-UnicodeCase">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCase</strong></code><span class="doc-sig-links"><a href="#t-UnicodeCase" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

One of the three simple, single-scalar case mappings.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnicodeUpper"><code><strong class="tok-constructor">UnicodeUpper</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a></code></li><li id="k-UnicodeLower"><code><strong class="tok-constructor">UnicodeLower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a></code></li><li id="k-UnicodeTitle"><code><strong class="tok-constructor">UnicodeTitle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeCaseError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCaseError</strong></code><span class="doc-sig-links"><a href="#t-UnicodeCaseError" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L370" title="View source at line 370">Source</a></span></div>
<div class="doc-body">

A precise invalid custom-case rule, carrying its zero-based rule index.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnicodeCaseInvalidBounds"><code><strong class="tok-constructor">UnicodeCaseInvalidBounds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a></code></li><li id="k-UnicodeCaseInvalidOrder"><code><strong class="tok-constructor">UnicodeCaseInvalidOrder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a></code></li><li id="k-UnicodeCaseInvalidMapping"><code><strong class="tok-constructor">UnicodeCaseInvalidMapping</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a></code></li><li id="k-UnicodeCaseInvalidAlternating"><code><strong class="tok-constructor">UnicodeCaseInvalidAlternating</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a></code></li><li id="k-UnicodeCaseTooManyRules"><code><strong class="tok-constructor">UnicodeCaseTooManyRules</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeCaseRange">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeCaseRange</strong></code><span class="doc-sig-links"><a href="#t-UnicodeCaseRange" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L345" title="View source at line 345">Source</a></span></div>
<div class="doc-body">

A complete inclusive simple-case rule with signed upper, lower and title deltas.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkUnicodeCaseRange"><code><strong class="tok-constructor">MkUnicodeCaseRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UnicodeSpecialCase">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnicodeSpecialCase</strong></code><span class="doc-sig-links"><a href="#t-UnicodeSpecialCase" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L392" title="View source at line 392">Source</a></span></div>
<div class="doc-body">

An immutable locale override set; explicit identity deltas suppress default mappings.

</div>
</article>

# Values

<article class="doc-entry" id="v-unicode-azeri-case">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-azeri-case</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code><span class="doc-sig-links"><a href="#v-unicode-azeri-case" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L403" title="View source at line 403">Source</a></span></div>
<div class="doc-body">

Azerbaijani simple-case overrides, sharing the immutable Turkic rules.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-count" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Number of independent simple-case mapping selectors.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-error-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-error-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-case-error-index" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L383" title="View source at line 383">Source</a></span></div>
<div class="doc-body">

Observe the zero-based index of an invalid custom-case rule.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-index" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Observe the stable numeric selector associated with one case.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-at" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L572" title="View source at line 572">Source</a></span></div>
<div class="doc-body">

Observe one standard case rule; out-of-range indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-count" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L581" title="View source at line 581">Source</a></span></div>
<div class="doc-body">

Count complete standard case range records without allocating their observations.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-high">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-high</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-high" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L354" title="View source at line 354">Source</a></span></div>
<div class="doc-body">

The upper inclusive source code point of a case rule.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-low">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-low</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-low" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L350" title="View source at line 350">Source</a></span></div>
<div class="doc-body">

The lower inclusive source code point of a case rule.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-lower" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L362" title="View source at line 362">Source</a></span></div>
<div class="doc-body">

The signed lowercase delta, or the alternating-case sentinel.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-title">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-title</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-title" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L366" title="View source at line 366">Source</a></span></div>
<div class="doc-body">

The signed titlecase delta, or the alternating-case sentinel.

</div>
</article>

<article class="doc-entry" id="v-unicode-case-range-upper">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-case-range-upper</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-case-range-upper" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L358" title="View source at line 358">Source</a></span></div>
<div class="doc-body">

The signed uppercase delta, or the alternating-case sentinel.

</div>
</article>

<article class="doc-entry" id="v-unicode-equal-fold">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-equal-fold</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-equal-fold" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L210" title="View source at line 210">Source</a></span></div>
<div class="doc-body">

Test whether two scalars belong to the same simple-fold cycle.

</div>
</article>

<article class="doc-entry" id="v-unicode-equal-fold-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-equal-fold-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-equal-fold-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L204" title="View source at line 204">Source</a></span></div>
<div class="doc-body">

Test numeric simple-fold equivalence; identical invalid inputs compare equal.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-control">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-control</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-control" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L218" title="View source at line 218">Source</a></span></div>
<div class="doc-body">

Test the Unicode control class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-control-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-control-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-control-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L214" title="View source at line 214">Source</a></span></div>
<div class="doc-body">

Test the Unicode control class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-digit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-digit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-digit" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L226" title="View source at line 226">Source</a></span></div>
<div class="doc-body">

Test the Unicode digit class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-digit-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-digit-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-digit-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L222" title="View source at line 222">Source</a></span></div>
<div class="doc-body">

Test the Unicode digit class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-graphic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-graphic" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L298" title="View source at line 298">Source</a></span></div>
<div class="doc-body">

Test the Unicode graphic class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-graphic-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-graphic-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-graphic-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L294" title="View source at line 294">Source</a></span></div>
<div class="doc-body">

Test the Unicode graphic class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-letter">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-letter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-letter" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L234" title="View source at line 234">Source</a></span></div>
<div class="doc-body">

Test the Unicode letter class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-letter-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-letter-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-letter-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L230" title="View source at line 230">Source</a></span></div>
<div class="doc-body">

Test the Unicode letter class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-lower" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L242" title="View source at line 242">Source</a></span></div>
<div class="doc-body">

Test the Unicode lower class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-lower-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-lower-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L238" title="View source at line 238">Source</a></span></div>
<div class="doc-body">

Test the Unicode lower class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-mark">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-mark</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-mark" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L266" title="View source at line 266">Source</a></span></div>
<div class="doc-body">

Test the Unicode mark class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-mark-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-mark-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-mark-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L262" title="View source at line 262">Source</a></span></div>
<div class="doc-body">

Test the Unicode mark class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-number" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L274" title="View source at line 274">Source</a></span></div>
<div class="doc-body">

Test the Unicode number class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-number-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-number-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-number-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L270" title="View source at line 270">Source</a></span></div>
<div class="doc-body">

Test the Unicode number class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-print">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-print</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-print" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L306" title="View source at line 306">Source</a></span></div>
<div class="doc-body">

Test the Unicode print class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-print-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-print-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-print-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L302" title="View source at line 302">Source</a></span></div>
<div class="doc-body">

Test the Unicode print class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-punct">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-punct</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-punct" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L282" title="View source at line 282">Source</a></span></div>
<div class="doc-body">

Test the Unicode punct class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-punct-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-punct-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-punct-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L278" title="View source at line 278">Source</a></span></div>
<div class="doc-body">

Test the Unicode punct class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-scalar-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-scalar-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-scalar-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

Test a signed numeric value for Unicode scalar validity, excluding surrogates.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-space">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-space</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-space" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L317" title="View source at line 317">Source</a></span></div>
<div class="doc-body">

Test whether one scalar is Unicode whitespace.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-space-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-space-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-space-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L310" title="View source at line 310">Source</a></span></div>
<div class="doc-body">

Test Unicode whitespace, including Latin-1 separators and the full White_Space property.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-symbol">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-symbol</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-symbol" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L290" title="View source at line 290">Source</a></span></div>
<div class="doc-body">

Test the Unicode symbol class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-symbol-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-symbol-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-symbol-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L286" title="View source at line 286">Source</a></span></div>
<div class="doc-body">

Test the Unicode symbol class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-title">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-title</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-title" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L258" title="View source at line 258">Source</a></span></div>
<div class="doc-body">

Test the Unicode title class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-title-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-title-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L254" title="View source at line 254">Source</a></span></div>
<div class="doc-body">

Test the Unicode title class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-upper">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-upper</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-upper" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L250" title="View source at line 250">Source</a></span></div>
<div class="doc-body">

Test the Unicode upper class of one valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-is-upper-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-is-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-is-upper-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L246" title="View source at line 246">Source</a></span></div>
<div class="doc-body">

Test the Unicode upper class for a signed code point, rejecting numbers outside the Unicode domain.

</div>
</article>

<article class="doc-entry" id="v-unicode-max-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-max-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-max-ascii" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

Highest ASCII code point.

</div>
</article>

<article class="doc-entry" id="v-unicode-max-codepoint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-max-codepoint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-max-codepoint" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

Highest Unicode code point; this bound alone does not exclude surrogates.

</div>
</article>

<article class="doc-entry" id="v-unicode-max-latin1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-max-latin1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-max-latin1" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

Highest Latin-1 code point.

</div>
</article>

<article class="doc-entry" id="v-unicode-replacement">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-replacement</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-replacement" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

The Unicode replacement scalar U+FFFD.

</div>
</article>

<article class="doc-entry" id="v-unicode-simple-fold">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-simple-fold</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-simple-fold" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L194" title="View source at line 194">Source</a></span></div>
<div class="doc-body">

Return the next scalar in its simple-fold cycle without normalization or expansion.

</div>
</article>

<article class="doc-entry" id="v-unicode-simple-fold-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-simple-fold-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-simple-fold-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L184" title="View source at line 184">Source</a></span></div>
<div class="doc-body">

Return the numeric-order successor in a simple-fold cycle; invalid numbers remain unchanged.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-affects-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-affects-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unicode-special-affects-ascii" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L609" title="View source at line 609">Source</a></span></div>
<div class="doc-body">

Whether any override can affect ASCII, allowing bulk default ASCII mapping when false.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-case">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-case</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseError" title="Data.Unicode.Kernel.UnicodeCaseError">UnicodeCaseError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code><span class="doc-sig-links"><a href="#v-unicode-special-case" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L451" title="View source at line 451">Source</a></span></div>
<div class="doc-body">

Construct scalar-preserving ordered custom rules, with identity rules overriding default casing.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-case-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-case-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-unicode-special-case-count" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L553" title="View source at line 553">Source</a></span></div>
<div class="doc-body">

Count immutable override records without materializing their observations.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-case-range-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-case-range-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCaseRange" title="Data.Unicode.Kernel.UnicodeCaseRange">UnicodeCaseRange</a></code><span class="doc-sig-links"><a href="#v-unicode-special-case-range-at" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L566" title="View source at line 566">Source</a></span></div>
<div class="doc-body">

Observe one override record; out-of-range indices return None.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L549" title="View source at line 549">Source</a></span></div>
<div class="doc-body">

Apply one scalar-preserving locale override mapping to a valid character.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-lower" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L597" title="View source at line 597">Source</a></span></div>
<div class="doc-body">

Apply scalar-preserving locale lowercase overrides before default casing.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-lower-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-lower-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L593" title="View source at line 593">Source</a></span></div>
<div class="doc-body">

Apply locale lowercase overrides to a numeric code point, preserving invalid inputs.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-title">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-title</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-title" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L605" title="View source at line 605">Source</a></span></div>
<div class="doc-body">

Apply scalar-preserving locale titlecase overrides before default casing.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-title-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-title-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L601" title="View source at line 601">Source</a></span></div>
<div class="doc-body">

Apply locale titlecase overrides to a numeric code point, preserving invalid inputs.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-upper">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-upper</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-upper" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L589" title="View source at line 589">Source</a></span></div>
<div class="doc-body">

Apply scalar-preserving locale uppercase overrides before default casing.

</div>
</article>

<article class="doc-entry" id="v-unicode-special-to-upper-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-special-to-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-special-to-upper-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L585" title="View source at line 585">Source</a></span></div>
<div class="doc-body">

Apply locale uppercase overrides to a numeric code point, preserving invalid inputs.

</div>
</article>

<article class="doc-entry" id="v-unicode-to">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeCase" title="Data.Unicode.Kernel.UnicodeCase">UnicodeCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-to" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Apply one simple case mapping to a valid scalar.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-to-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Map a signed selector and numeric code point; an invalid selector returns U+FFFD.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-to-lower" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L333" title="View source at line 333">Source</a></span></div>
<div class="doc-body">

Return the simple single-scalar lowercase mapping.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-lower-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-lower-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-to-lower-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L329" title="View source at line 329">Source</a></span></div>
<div class="doc-body">

Return the simple lowercase mapping of a numeric code point; invalid inputs remain unchanged.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-title">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-title</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-to-title" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L341" title="View source at line 341">Source</a></span></div>
<div class="doc-body">

Return the simple single-scalar titlecase mapping.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-title-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-title-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-to-title-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L337" title="View source at line 337">Source</a></span></div>
<div class="doc-body">

Return the simple titlecase mapping of a numeric code point; invalid inputs remain unchanged.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-upper">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-upper</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unicode-to-upper" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L325" title="View source at line 325">Source</a></span></div>
<div class="doc-body">

Return the simple single-scalar uppercase mapping.

</div>
</article>

<article class="doc-entry" id="v-unicode-to-upper-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-to-upper-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-to-upper-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L321" title="View source at line 321">Source</a></span></div>
<div class="doc-body">

Return the simple uppercase mapping of a numeric code point; invalid inputs remain unchanged.

</div>
</article>

<article class="doc-entry" id="v-unicode-turkish-case">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-turkish-case</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Unicode/#t-UnicodeSpecialCase" title="Data.Unicode.Kernel.UnicodeSpecialCase">UnicodeSpecialCase</a></code><span class="doc-sig-links"><a href="#v-unicode-turkish-case" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L399" title="View source at line 399">Source</a></span></div>
<div class="doc-body">

Turkish simple-case overrides, taking precedence over default mappings.

</div>
</article>

<article class="doc-entry" id="v-unicode-upper-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-upper-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-unicode-upper-lower" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Sentinel delta used by alternating uppercase/lowercase range observations.

</div>
</article>

<article class="doc-entry" id="v-unicode-version-major">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-version-major</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-version-major" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Major edition of the immutable classification and mapping data.

</div>
</article>

<article class="doc-entry" id="v-unicode-version-minor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-version-minor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-version-minor" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

Minor edition of the immutable classification and mapping data.

</div>
</article>

<article class="doc-entry" id="v-unicode-version-patch">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unicode-version-patch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-unicode-version-patch" title="Permalink">#</a><a href="/packages/stdlib/Data.Unicode.Kernel/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Patch edition of the immutable classification and mapping data.

</div>
</article>
