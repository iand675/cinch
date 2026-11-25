# Text.Scan

Scanning primitives over input text and the shared position/error vocabulary.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Scan</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Scan/source/">View source</a><a href="/packages/search/?q=%2BText.Scan">Search module</a><a href="/reference-sources/Text.Scan.cinch" download="Text.Scan.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Text.Scan: the literal-string scanning and formatting utilities
(the Ruby String surface, redesigned where our principles say so).

SCANNING (the subject-last argument order throughout: the needle,
separator or pattern names what happens, the subject is last):
  `string-index-of` / `string-contains` / `string-starts-with` /
  `string-ends-with` over scalar indices; `string-char-index-of`
  for a single scalar; `string-split-on` (an empty separator splits
  into single scalars, Ruby's `split("")`); `string-join`;
  `string-lines` (splits at LF; a single trailing empty line is
  dropped) and its inverse `string-unlines` (each line followed by
  LF); `string-words` (ASCII whitespace runs) and its inverse
  `string-unwords` (single-space join); `string-take` /
  `string-drop` / `string-split-at` cutting at a scalar count, and
  `string-take-while` / `string-drop-while` cutting at a scalar
  predicate. Prefix and suffix tests ARE `string-starts-with` /
  `string-ends-with` (no separate `is-prefix-of` spelling).

FORMATTING: `format` is the template interpolation (`{}` consumes
  the next argument in order, `{{` and `}}` are literal braces;
  this replaces sprintf-style `%` formatting on purpose: the
  arguments' types do the rendering through `Text.Show`).
  `string-pad-left` / `string-pad-right` / `string-center` pad to
  a scalar width with spaces; `string-strip` / `string-lstrip` /
  `string-rstrip` / `string-chomp` / `string-chop` trim; the case
  family maps ASCII letters only. The regex-driven scanning family
  (scan, split, replace) rides the engine in `Text.Regex`.

UNICODE SCOPE, named honestly at the UTF-8 layers' boundary:
every operation here is scalar-value aware (indices are scalar
indices, lengths are scalar counts, slices cut at scalar
boundaries). What is NOT here: canonical normalization
(NFC/NFD/NFKC/NFKD) and grapheme clusters (the UTF-8 machinery
carries no Unicode character database), Unicode or locale-aware
case mapping (`string-upcase-ascii` and friends map A-Z/a-z only
and leave every other scalar untouched), and non-ASCII whitespace
classes. `string-normalize-newlines` is the one normalization
provided: CRLF and lone CR line endings fold to LF.

</details>

<details class="doc-synopsis"><summary>Synopsis (31 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-FormatError">FormatError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FormatError</strong></code></li><li><a class="syn-name" href="#v-format">format</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">format</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Scan/#t-FormatError" title="Text.Scan.FormatError">FormatError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-render-format-error">render-format-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-format-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Scan/#t-FormatError" title="Text.Scan.FormatError">FormatError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-capitalize-ascii">string-capitalize-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-capitalize-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-center">string-center</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-center</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-char-index-of">string-char-index-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-char-index-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-string-chomp">string-chomp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-chomp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-chop">string-chop</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-chop</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-contains">string-contains</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-contains</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-string-downcase-ascii">string-downcase-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-downcase-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-drop">string-drop</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-drop</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-drop-while">string-drop-while</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-drop-while</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-ends-with">string-ends-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-ends-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-string-index-of">string-index-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-index-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-string-join">string-join</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-lines">string-lines</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-lines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-lstrip">string-lstrip</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-lstrip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-normalize-newlines">string-normalize-newlines</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-normalize-newlines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-pad-left">string-pad-left</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-pad-left</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-pad-right">string-pad-right</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-pad-right</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-rstrip">string-rstrip</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-rstrip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-split-at">string-split-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-split-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-string-split-on">string-split-on</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-split-on</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-starts-with">string-starts-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-starts-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-string-strip">string-strip</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-strip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-take">string-take</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-take</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-take-while">string-take-while</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-take-while</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-unlines">string-unlines</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-unlines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-unwords">string-unwords</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-unwords</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-upcase-ascii">string-upcase-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-upcase-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-words">string-words</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-words</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-FormatError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FormatError</strong></code><span class="doc-sig-links"><a href="#t-FormatError" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L401" title="View source at line 401">Source</a></span></div>
<div class="doc-body">

`format`: template interpolation over `{}` holes (the sprintf
replacement; the arguments render themselves at their own
types).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MissingFormatArgument"><code><strong class="tok-constructor">MissingFormatArgument</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Scan/#t-FormatError" title="Text.Scan.FormatError">FormatError</a></code></li><li id="k-UnusedFormatArgument"><code><strong class="tok-constructor">UnusedFormatArgument</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Scan/#t-FormatError" title="Text.Scan.FormatError">FormatError</a></code></li><li id="k-UnmatchedFormatBrace"><code><strong class="tok-constructor">UnmatchedFormatBrace</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Scan/#t-FormatError" title="Text.Scan.FormatError">FormatError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-format">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">format</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Scan/#t-FormatError" title="Text.Scan.FormatError">FormatError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-format" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L412" title="View source at line 412">Source</a></span></div>
</article>

<article class="doc-entry" id="v-render-format-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-format-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Scan/#t-FormatError" title="Text.Scan.FormatError">FormatError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-format-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L406" title="View source at line 406">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-capitalize-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-capitalize-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-capitalize-ascii" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L336" title="View source at line 336">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-center">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-center</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-center" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L362" title="View source at line 362">Source</a></span></div>
<div class="doc-body">

Center in `w` scalars; the extra space of an odd pad goes right.

</div>
</article>

<article class="doc-entry" id="v-string-char-index-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-char-index-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-string-char-index-of" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L154" title="View source at line 154">Source</a></span></div>
<div class="doc-body">

The scalar index of the first occurrence of a single scalar (the
one-scalar special case of `string-index-of`; `None` when the
scalar is absent, so it never coincides with the empty-needle
rule above).

</div>
</article>

<article class="doc-entry" id="v-string-chomp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-chomp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-chomp" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L309" title="View source at line 309">Source</a></span></div>
<div class="doc-body">

Drop one trailing line ending: LF, CRLF or CR.

</div>
</article>

<article class="doc-entry" id="v-string-chop">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-chop</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-chop" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L323" title="View source at line 323">Source</a></span></div>
<div class="doc-body">

Drop the last scalar.

</div>
</article>

<article class="doc-entry" id="v-string-contains">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-contains</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-string-contains" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L145" title="View source at line 145">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-downcase-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-downcase-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-downcase-ascii" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L333" title="View source at line 333">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-drop">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-drop</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-drop" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L268" title="View source at line 268">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-drop-while">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-drop-while</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-drop-while" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L277" title="View source at line 277">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-ends-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-ends-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-string-ends-with" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L165" title="View source at line 165">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-index-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-index-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-string-index-of" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

The scalar index where `needle` first occurs in `s` (an empty
needle occurs at 0).

</div>
</article>

<article class="doc-entry" id="v-string-join">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-join" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L204" title="View source at line 204">Source</a></span></div>
<div class="doc-body">

Join with a separator between every pair.

</div>
</article>

<article class="doc-entry" id="v-string-lines">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-lines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-lines" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L217" title="View source at line 217">Source</a></span></div>
<div class="doc-body">

The lines of a string: the segments between LF scalars, with one
trailing empty segment dropped when the string ends in LF (an
empty string has no lines).

</div>
</article>

<article class="doc-entry" id="v-string-lstrip">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-lstrip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-lstrip" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L297" title="View source at line 297">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-normalize-newlines">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-normalize-newlines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-normalize-newlines" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L378" title="View source at line 378">Source</a></span></div>
<div class="doc-body">

Line-ending normalization (the one normalization the UTF-8
layers permit: CRLF and lone CR fold to LF).

</div>
</article>

<article class="doc-entry" id="v-string-pad-left">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-pad-left</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-pad-left" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L351" title="View source at line 351">Source</a></span></div>
<div class="doc-body">

Padding (scalar widths, spaces).

</div>
</article>

<article class="doc-entry" id="v-string-pad-right">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-pad-right</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-pad-right" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L356" title="View source at line 356">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-rstrip">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-rstrip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-rstrip" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L300" title="View source at line 300">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-split-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-split-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-string-split-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L262" title="View source at line 262">Source</a></span></div>
<div class="doc-body">

Splitting at a scalar count, over scalar boundaries (a count past
the end puts everything in the first part; 0 puts everything in
the second).

</div>
</article>

<article class="doc-entry" id="v-string-split-on">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-split-on</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-split-on" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L172" title="View source at line 172">Source</a></span></div>
<div class="doc-body">

Split at every occurrence of the separator (the separators are
removed). Trailing empty pieces are dropped (Ruby's default) and
an empty separator splits into single-scalar strings (Ruby's
`split("")`).

</div>
</article>

<article class="doc-entry" id="v-string-starts-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-starts-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-string-starts-with" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L162" title="View source at line 162">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-strip">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-strip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-strip" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L294" title="View source at line 294">Source</a></span></div>
<div class="doc-body">

Trimming and case (ASCII).

</div>
</article>

<article class="doc-entry" id="v-string-take">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-take</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-take" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L265" title="View source at line 265">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-take-while">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-take-while</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-take-while" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L274" title="View source at line 274">Source</a></span></div>
<div class="doc-body">

The longest prefix (suffix) whose scalars all satisfy `p`; unlike
`string-take`/`string-drop` the cut is predicate-driven, so the
caller supplies the scalar test.

</div>
</article>

<article class="doc-entry" id="v-string-unlines">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-unlines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-unlines" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L246" title="View source at line 246">Source</a></span></div>
<div class="doc-body">

The inverse of `string-lines`: every line followed by LF, so
`string-unlines (string-lines s)` is `s` with exactly one trailing
LF (and the empty list is the empty string, matching
`string-lines ""`). It cannot be a bare join: a join would leave
the last line unterminated.

</div>
</article>

<article class="doc-entry" id="v-string-unwords">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-unwords</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-unwords" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L256" title="View source at line 256">Source</a></span></div>
<div class="doc-body">

The inverse of `string-words`: a single space between elements
(the empty list is the empty string, matching `string-words ""`).

</div>
</article>

<article class="doc-entry" id="v-string-upcase-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-upcase-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-upcase-ascii" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L330" title="View source at line 330">Source</a></span></div>
<div class="doc-body">

ASCII case mapping only (the honest boundary: no Unicode case
data exists in the UTF-8 layers).

</div>
</article>

<article class="doc-entry" id="v-string-words">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-words</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-words" title="Permalink">#</a><a href="/packages/stdlib/Text.Scan/source/#L229" title="View source at line 229">Source</a></span></div>
<div class="doc-body">

The whitespace-separated words (ASCII whitespace runs delimit).

</div>
</article>
