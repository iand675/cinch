# Text.Regex

Regular-expression parsing/matching and replacement under the module's admitted regex language.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Regex</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Regex/">View source</a><a href="/library/search/?q=%2BText.Regex">Search module</a><a href="/reference-sources/Text.Regex.cinch" download="Text.Regex.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Text.Regex`: [Text.Regex](Text.Regex.md).

```sh
cp seed/prelude/Text/Regex.cinch ./Text.Regex.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Text.Regex: RE2-style regular expressions with linear-time matching.

The engine is a Pike VM (a Thompson NFA simulation with capture
slots and thread priority): one pass over the input, at most one
thread per program instruction per position, so a match costs
O(input * program) instruction dispatches BY CONSTRUCTION and no
input can make it cost more. A backtracking engine's ReDoS surface
(the classic `(a+)+b` against a long run of `a`) does not exist
here: there is no backtracking to blow up. The claim is pinned
step-for-step and against the clock in `seed/tests/regex.rs`.

THE PATTERN LANGUAGE (the RE2 subset):
  literals and escapes: `\n \r \t \f \v`, `\<punctuation>` as the
    literal character, `\xHH` and `\x{HEX}` for scalar values;
  classes: `[abc]`, `[^abc]`, ranges `[a-z]`, the shorthands
    `\d \D \w \W \s \S`; a `]` first in the class is a literal,
    a `-` first or last is a literal;
  `.` matches one Unicode scalar value except LF (this subset has
    no flag surface, so `.` never matches LF);
  repetition: `*` `+` `?`, `{n}` `{n,}` `{n,m}` (each bound at
    most 1000), each with a lazy `?` suffix (`a*?`, `a{2,4}?`);
  alternation `|`, grouping `( ... )` and `(?: ... )`;
  captures: numbered groups; group 0 is the whole match;
  anchors: `^` `\$` at the text start and end (no multiline mode),
    `\b` `\B` at word boundaries (ASCII word characters).

SEMANTICS are leftmost-first, the Ruby/Perl order: the earliest
start position wins and, at that position, the first alternative
in backtracking order wins (`a` before `ab` in `(a|ab)`), with
greedy quantifiers before lazy ones. Capture slots record the
winning (highest priority) path, exactly as a backtracking engine
would report them.

NAMED REFUSALS (each error names itself; nothing silently
misbehaves): backreferences and digit escapes (`\1`, `\0`:
backreference-refused), lookahead and lookbehind (`(?=`, `(?!`,
`(?<=`, `(?<!`: lookaround-refused), named capture groups
(`(?<name>`, `(?P<name>`: named-group-refused), inline flags and
other `(?` extensions (flag-group-refused), possessive quantifiers
(`a*+`: possessive-refused), nested quantifiers (`a**`:
nested-repeat-refused), quantifiers over empty-width assertions
(`^*`: empty-width-repeat-refused), `\p{...}` Unicode property
classes (unicode-property-refused: they need the Unicode character
database, which the UTF-8 layers do not carry), the `\A \Z \z \G`
anchor aliases and every other unknown letter escape
(unsupported-escape), malformed counts (`a{2,1}`:
invalid-repeat-range; `a{`: invalid-repeat-syntax), bounds above
the 1000 repeat cap (repeat-limit-exceeded), programs above 65536
instructions after repeat expansion (program-too-large), and the
structural errors: unterminated-class, unterminated-group,
unmatched-close-paren, trailing-backslash, invalid-hex-escape,
invalid-class-range, dangling-quantifier.

UNICODE SCOPE (what the UTF-8 layers permit, and what they do
not): matching operates on Unicode scalar values, never bytes:
`.` consumes one scalar, classes range over scalar values,
captures span scalar boundaries, `\x{...}` names any scalar, and
match spans report byte offsets into the UTF-8 subject. What is
NOT here, because the UTF-8 machinery carries no Unicode character
database: canonical normalization (NFC/NFD/NFKC/NFKD), grapheme
clusters, locale-aware casing, and `\p{...}` property classes.
Precomposed and decomposed spellings of one grapheme are different
inputs to this engine; the boundary is pinned in the tests.

Module scheme (design/base-library.md): `Text.*` is text
processing over String/Bytes. The regex-driven scanning family
(scan, split, replace) rides on this engine and lives here beside
it; `Text.Scan` holds the literal-string scanning and formatting
utilities.

</details>

<details class="doc-synopsis"><summary>Synopsis (22 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Match">Match</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Match</strong></code></li><li><a class="syn-name" href="#t-Regex">Regex</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Regex</strong></code></li><li><a class="syn-name" href="#t-RegexError">RegexError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RegexError</strong></code></li><li><a class="syn-name" href="#v-match-end">match-end</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">match-end</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-match-group">match-group</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">match-group</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-match-group-count">match-group-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">match-group-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-match-start">match-start</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">match-start</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-match-text">match-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">match-text</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-parse-regex">parse-regex</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-regex</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a></code></li><li><a class="syn-name" href="#v-regex-count">regex-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-regex-escape">regex-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-escape</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-regex-match">regex-match</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-match</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a></code></li><li><a class="syn-name" href="#v-regex-match-counted">regex-match-counted</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-match-counted</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-regex-replace-all">regex-replace-all</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-replace-all</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-regex-replace-all-template">regex-replace-all-template</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-replace-all-template</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-regex-replace-first">regex-replace-first</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-replace-first</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-regex-replace-first-template">regex-replace-first-template</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-replace-first-template</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-regex-scan">regex-scan</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-scan</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a></code></li><li><a class="syn-name" href="#v-regex-scan-texts">regex-scan-texts</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-scan-texts</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-regex-search-from">regex-search-from</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-search-from</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a></code></li><li><a class="syn-name" href="#v-regex-split">regex-split</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">regex-split</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-render-regex-error">render-regex-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-regex-error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Match">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Match</strong></code><span class="doc-sig-links"><a href="#t-Match" title="Permalink">#</a><a href="/library/source/Text.Regex/#L895" title="View source at line 895">Source</a></span></div>
<div class="doc-body">

A match: byte offsets into the subject and one text per group
(index 0 is the whole match; a group that did not participate is
None).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Match"><code><strong class="tok-constructor">Match</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Regex">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Regex</strong></code><span class="doc-sig-links"><a href="#t-Regex" title="Permalink">#</a><a href="/library/source/Text.Regex/#L798" title="View source at line 798">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Regex"><code><strong class="tok-constructor">Regex</strong> :: <span class="tok-constructor">Prog</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-RegexError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RegexError</strong></code><span class="doc-sig-links"><a href="#t-RegexError" title="Permalink">#</a><a href="/library/source/Text.Regex/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

The named refusals and structural errors: one constructor per
refusal so each error names itself, with the pattern's scalar
offset for the report.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-BackreferenceRefused"><code><strong class="tok-constructor">BackreferenceRefused</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-LookaroundRefused"><code><strong class="tok-constructor">LookaroundRefused</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-NamedGroupRefused"><code><strong class="tok-constructor">NamedGroupRefused</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-FlagGroupRefused"><code><strong class="tok-constructor">FlagGroupRefused</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-UnicodePropertyRefused"><code><strong class="tok-constructor">UnicodePropertyRefused</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-UnsupportedEscape"><code><strong class="tok-constructor">UnsupportedEscape</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-PossessiveRefused"><code><strong class="tok-constructor">PossessiveRefused</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-NestedRepeatRefused"><code><strong class="tok-constructor">NestedRepeatRefused</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-EmptyWidthRepeatRefused"><code><strong class="tok-constructor">EmptyWidthRepeatRefused</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-DanglingQuantifier"><code><strong class="tok-constructor">DanglingQuantifier</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-RepeatLimitExceeded"><code><strong class="tok-constructor">RepeatLimitExceeded</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-InvalidRepeatSyntax"><code><strong class="tok-constructor">InvalidRepeatSyntax</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-InvalidRepeatRange"><code><strong class="tok-constructor">InvalidRepeatRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-InvalidClassRange"><code><strong class="tok-constructor">InvalidClassRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-UnterminatedClass"><code><strong class="tok-constructor">UnterminatedClass</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-UnterminatedGroup"><code><strong class="tok-constructor">UnterminatedGroup</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-UnmatchedCloseParen"><code><strong class="tok-constructor">UnmatchedCloseParen</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-TrailingBackslash"><code><strong class="tok-constructor">TrailingBackslash</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-InvalidHexEscape"><code><strong class="tok-constructor">InvalidHexEscape</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-ProgramTooLarge"><code><strong class="tok-constructor">ProgramTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li><li id="k-InvalidPatternSyntax"><code><strong class="tok-constructor">InvalidPatternSyntax</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-match-end">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">match-end</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-match-end" title="Permalink">#</a><a href="/library/source/Text.Regex/#L902" title="View source at line 902">Source</a></span></div>
</article>

<article class="doc-entry" id="v-match-group">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">match-group</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-match-group" title="Permalink">#</a><a href="/library/source/Text.Regex/#L914" title="View source at line 914">Source</a></span></div>
</article>

<article class="doc-entry" id="v-match-group-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">match-group-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-match-group-count" title="Permalink">#</a><a href="/library/source/Text.Regex/#L920" title="View source at line 920">Source</a></span></div>
</article>

<article class="doc-entry" id="v-match-start">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">match-start</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-match-start" title="Permalink">#</a><a href="/library/source/Text.Regex/#L898" title="View source at line 898">Source</a></span></div>
</article>

<article class="doc-entry" id="v-match-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">match-text</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-match-text" title="Permalink">#</a><a href="/library/source/Text.Regex/#L906" title="View source at line 906">Source</a></span></div>
</article>

<article class="doc-entry" id="v-parse-regex">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-regex</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a></code><span class="doc-sig-links"><a href="#v-parse-regex" title="Permalink">#</a><a href="/library/source/Text.Regex/#L928" title="View source at line 928">Source</a></span></div>
<div class="doc-body">

Compile a pattern. The parser is strict about quantifier
position: `{` right after an atom MUST be a valid quantifier
(escape it as `\{` for a literal brace there); a bare `{`
elsewhere is a literal.

</div>
</article>

<article class="doc-entry" id="v-regex-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-regex-count" title="Permalink">#</a><a href="/library/source/Text.Regex/#L1322" title="View source at line 1322">Source</a></span></div>
<div class="doc-body">

Count the matches (the scan's length).

</div>
</article>

<article class="doc-entry" id="v-regex-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-escape</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-regex-escape" title="Permalink">#</a><a href="/library/source/Text.Regex/#L945" title="View source at line 945">Source</a></span></div>
<div class="doc-body">

Escape the regex metacharacters in a literal string, so the
result matches exactly that string (Ruby's Regexp.escape).

</div>
</article>

<article class="doc-entry" id="v-regex-match">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-match</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a></code><span class="doc-sig-links"><a href="#v-regex-match" title="Permalink">#</a><a href="/library/source/Text.Regex/#L1230" title="View source at line 1230">Source</a></span></div>
<div class="doc-body">

The leftmost-first match of the pattern anywhere in the string.

</div>
</article>

<article class="doc-entry" id="v-regex-match-counted">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-match-counted</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-regex-match-counted" title="Permalink">#</a><a href="/library/source/Text.Regex/#L1235" title="View source at line 1235">Source</a></span></div>
<div class="doc-body">

The match and the engine's VM dispatch count (the ReDoS pin:
this counter is the wall-clock-independent measure of work).

</div>
</article>

<article class="doc-entry" id="v-regex-replace-all">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-replace-all</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-regex-replace-all" title="Permalink">#</a><a href="/library/source/Text.Regex/#L1327" title="View source at line 1327">Source</a></span></div>
<div class="doc-body">

Replace every match with `f` applied to it (the text between
matches is copied through unchanged).

</div>
</article>

<article class="doc-entry" id="v-regex-replace-all-template">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-replace-all-template</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-regex-replace-all-template" title="Permalink">#</a><a href="/library/source/Text.Regex/#L1352" title="View source at line 1352">Source</a></span></div>
</article>

<article class="doc-entry" id="v-regex-replace-first">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-replace-first</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-regex-replace-first" title="Permalink">#</a><a href="/library/source/Text.Regex/#L1332" title="View source at line 1332">Source</a></span></div>
</article>

<article class="doc-entry" id="v-regex-replace-first-template">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-replace-first-template</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-regex-replace-first-template" title="Permalink">#</a><a href="/library/source/Text.Regex/#L1357" title="View source at line 1357">Source</a></span></div>
</article>

<article class="doc-entry" id="v-regex-scan">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-scan</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a></code><span class="doc-sig-links"><a href="#v-regex-scan" title="Permalink">#</a><a href="/library/source/Text.Regex/#L1268" title="View source at line 1268">Source</a></span></div>
<div class="doc-body">

The scanning family: every non-overlapping match left to right.
After a non-empty match the scan resumes at its end; after an
empty match it resumes one scalar later; an empty match at the
end of the subject is the last one. (This is the traversal Ruby
documents for String#scan.)

</div>
</article>

<article class="doc-entry" id="v-regex-scan-texts">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-scan-texts</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-regex-scan-texts" title="Permalink">#</a><a href="/library/source/Text.Regex/#L1292" title="View source at line 1292">Source</a></span></div>
</article>

<article class="doc-entry" id="v-regex-search-from">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-search-from</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Match" title="Text.Regex.Match">Match</a></code><span class="doc-sig-links"><a href="#v-regex-search-from" title="Permalink">#</a><a href="/library/source/Text.Regex/#L1247" title="View source at line 1247">Source</a></span></div>
<div class="doc-body">

The leftmost-first match starting at or after byte offset
`from` (`^` still means the text start; `from` only bounds
where matches may begin).

</div>
</article>

<article class="doc-entry" id="v-regex-split">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">regex-split</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-regex-split" title="Permalink">#</a><a href="/library/source/Text.Regex/#L1303" title="View source at line 1303">Source</a></span></div>
<div class="doc-body">

Split at every match (the separators are removed; a captured
group is not echoed into the output). Trailing empty pieces are
dropped, Ruby's default.

</div>
</article>

<article class="doc-entry" id="v-render-regex-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-regex-error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Regex/#t-RegexError" title="Text.Regex.RegexError">RegexError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-regex-error" title="Permalink">#</a><a href="/library/source/Text.Regex/#L224" title="View source at line 224">Source</a></span></div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Regex.cinch`](/library/source/Text.Regex/). Source SHA-256: `35adaa7be29d6731181e5e84054b4ef90fa7a40623ae9fe6a1cc2b962b27f9d1`.
