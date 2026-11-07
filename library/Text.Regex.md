# Text.Regex

Regular-expression parsing/matching and replacement under the module's admitted regex language.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Regex`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Regex.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Regex](Text.Regex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Regex: RE2-style regular expressions with linear-time matching.

The engine is a Pike VM (a Thompson NFA simulation with capture
slots and thread priority): one pass over the input, at most one
thread per program instruction per position, so a match costs
O(input * program) instruction dispatches BY CONSTRUCTION and no
input can make it cost more. A backtracking engine's ReDoS surface
(the classic `(a+)+b` against a long run of `a`) does not exist
here: there is no backtracking to blow up. The claim is pinned
step-for-step and against the clock in `compiler/tests/regex.rs`.

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

# Types and constructors

<a id="type-4d61746368"></a>

## `Match`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Match</span></code></pre>

A match: byte offsets into the subject and one text per group
(index 0 is the whole match; a group that did not participate is
None).

[Source](/reference-sources/Text.Regex.cinch) · line 895

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Match</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">List</span> (<span class="tok-constructor">Option</span> <span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Match</span>)))</code></pre>

<a id="type-5265676578"></a>

## `Regex`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Regex</span></code></pre>

[Source](/reference-sources/Text.Regex.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Regex</span> :: (<span class="tok-constructor">Prog</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Regex</span>))</code></pre>

<a id="type-52656765784572726f72"></a>

## `RegexError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RegexError</span></code></pre>

The named refusals and structural errors: one constructor per
refusal so each error names itself, with the pattern's scalar
offset for the report.

[Source](/reference-sources/Text.Regex.cinch) · line 201

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BackreferenceRefused</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LookaroundRefused</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NamedGroupRefused</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlagGroupRefused</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodePropertyRefused</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnsupportedEscape</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PossessiveRefused</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NestedRepeatRefused</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">EmptyWidthRepeatRefused</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DanglingQuantifier</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RepeatLimitExceeded</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidRepeatSyntax</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidRepeatRange</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidClassRange</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnterminatedClass</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnterminatedGroup</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnmatchedCloseParen</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TrailingBackslash</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidHexEscape</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ProgramTooLarge</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">RegexError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidPatternSyntax</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">RegexError</span>))</code></pre>

# Values

<a id="value-6d617463682d656e64"></a>

## `match-end`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">match-end</span> :: (<span class="tok-constructor">Match</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Text.Regex.cinch)

<a id="value-6d617463682d67726f7570"></a>

## `match-group`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">match-group</span> :: (<span class="tok-constructor">Match</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Regex.cinch)

<a id="value-6d617463682d67726f75702d636f756e74"></a>

## `match-group-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">match-group-count</span> :: (<span class="tok-constructor">Match</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Text.Regex.cinch)

<a id="value-6d617463682d7374617274"></a>

## `match-start`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">match-start</span> :: (<span class="tok-constructor">Match</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Text.Regex.cinch)

<a id="value-6d617463682d74657874"></a>

## `match-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">match-text</span> :: (<span class="tok-constructor">Match</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Regex.cinch)

<a id="value-70617273652d7265676578"></a>

## `parse-regex`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-regex</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">RegexError</span> <span class="tok-constructor">Regex</span>)</code></pre>

Compile a pattern. The parser is strict about quantifier
position: `{` right after an atom MUST be a valid quantifier
(escape it as `\{` for a literal brace there); a bare `{`
elsewhere is a literal.

[Source](/reference-sources/Text.Regex.cinch) · line 928

<a id="value-72656765782d636f756e74"></a>

## `regex-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-count</span> :: (<span class="tok-constructor">Regex</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Count the matches (the scan's length).

[Source](/reference-sources/Text.Regex.cinch) · line 1322

<a id="value-72656765782d657363617065"></a>

## `regex-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-escape</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

Escape the regex metacharacters in a literal string, so the
result matches exactly that string (Ruby's Regexp.escape).

[Source](/reference-sources/Text.Regex.cinch) · line 945

<a id="value-72656765782d6d61746368"></a>

## `regex-match`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-match</span> :: (<span class="tok-constructor">Regex</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Match</span>))</code></pre>

The leftmost-first match of the pattern anywhere in the string.

[Source](/reference-sources/Text.Regex.cinch) · line 1230

<a id="value-72656765782d6d617463682d636f756e746564"></a>

## `regex-match-counted`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-match-counted</span> :: (<span class="tok-constructor">Regex</span> -&gt; (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Option</span> <span class="tok-constructor">Match</span>, <span class="tok-constructor">Nat</span>)))</code></pre>

The match and the engine's VM dispatch count (the ReDoS pin:
this counter is the wall-clock-independent measure of work).

[Source](/reference-sources/Text.Regex.cinch) · line 1235

<a id="value-72656765782d7265706c6163652d616c6c"></a>

## `regex-replace-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-replace-all</span> :: (<span class="tok-constructor">Regex</span> -&gt; ((<span class="tok-constructor">Match</span> -&gt; <span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">String</span>)))</code></pre>

Replace every match with `f` applied to it (the text between
matches is copied through unchanged).

[Source](/reference-sources/Text.Regex.cinch) · line 1327

<a id="value-72656765782d7265706c6163652d616c6c2d74656d706c617465"></a>

## `regex-replace-all-template`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-replace-all-template</span> :: (<span class="tok-constructor">Regex</span> -&gt; (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">String</span>)))</code></pre>

[Source](/reference-sources/Text.Regex.cinch)

<a id="value-72656765782d7265706c6163652d6669727374"></a>

## `regex-replace-first`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-replace-first</span> :: (<span class="tok-constructor">Regex</span> -&gt; ((<span class="tok-constructor">Match</span> -&gt; <span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">String</span>)))</code></pre>

[Source](/reference-sources/Text.Regex.cinch)

<a id="value-72656765782d7265706c6163652d66697273742d74656d706c617465"></a>

## `regex-replace-first-template`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-replace-first-template</span> :: (<span class="tok-constructor">Regex</span> -&gt; (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">String</span>)))</code></pre>

[Source](/reference-sources/Text.Regex.cinch)

<a id="value-72656765782d7363616e"></a>

## `regex-scan`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-scan</span> :: (<span class="tok-constructor">Regex</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">List</span> <span class="tok-constructor">Match</span>))</code></pre>

The scanning family: every non-overlapping match left to right.
After a non-empty match the scan resumes at its end; after an
empty match it resumes one scalar later; an empty match at the
end of the subject is the last one. (This is the traversal Ruby
documents for String#scan.)

[Source](/reference-sources/Text.Regex.cinch) · line 1268

<a id="value-72656765782d7363616e2d7465787473"></a>

## `regex-scan-texts`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-scan-texts</span> :: (<span class="tok-constructor">Regex</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">List</span> <span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Regex.cinch)

<a id="value-72656765782d7365617263682d66726f6d"></a>

## `regex-search-from`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-search-from</span> :: (<span class="tok-constructor">Regex</span> -&gt; (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Match</span>)))</code></pre>

The leftmost-first match starting at or after byte offset
`from` (`^` still means the text start; `from` only bounds
where matches may begin).

[Source](/reference-sources/Text.Regex.cinch) · line 1247

<a id="value-72656765782d73706c6974"></a>

## `regex-split`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">regex-split</span> :: (<span class="tok-constructor">Regex</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">List</span> <span class="tok-constructor">String</span>))</code></pre>

Split at every match (the separators are removed; a captured
group is not echoed into the output). Trailing empty pieces are
dropped, Ruby's default.

[Source](/reference-sources/Text.Regex.cinch) · line 1303

<a id="value-72656e6465722d72656765782d6572726f72"></a>

## `render-regex-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">render-regex-error</span> :: (<span class="tok-constructor">RegexError</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Regex.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Regex.cinch`. Source SHA-256: `ed3a6d5ed60e34766ff22fc61ce1590e0bca20b3e37e3bd3c12a147ece928391`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
