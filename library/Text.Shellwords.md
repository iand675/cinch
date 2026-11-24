# Text.Shellwords

Shell-style word splitting/joining; this is data parsing, not execution of a shell command.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Shellwords</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Shellwords/">View source</a><a href="/library/search/?q=%2BText.Shellwords">Search module</a><a href="/reference-sources/Text.Shellwords.cinch" download="Text.Shellwords.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Scan](Text.Scan.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Text.Shellwords`: [Text.Scan](Text.Scan.md), [Text.Shellwords](Text.Shellwords.md).

```sh
cp seed/prelude/Text/Scan.cinch ./Text.Scan.cinch
cp seed/prelude/Text/Shellwords.cinch ./Text.Shellwords.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Text.Shellwords: Bourne-shell word splitting and argument
escaping, the Ruby `Shellwords` surface (design/stdlib-parity-ruby.md
lists Shellwords under collections/text utilities to port), as a
`Text.*` conversion over `String`.

`shellwords-split` is Ruby's `Shellwords.split`: whitespace-separated
words, single quotes taking the enclosed scalars literally, double
quotes honouring a backslash escape, an unquoted backslash escaping
the next scalar, and a named error for an unterminated quote. It is a
tokenizer, not a shell: outside quotes every other metacharacter
(`|`, `*`, `$`, `>`, `#`) is an ordinary scalar.

Design law shape:
  round trip: `split (join (Cons s Nil)) == Ok (Cons s Nil)` for
    every `s` (the escaping family inverts splitting; asserted over
    hostile strings in seed/tests/lib_shellwords.rs);
  empty-word preservation: a quoted empty argument is an
    empty-string word (`split "''" == Ok (Cons "" Nil)` and the
    double-quoted form likewise), never dropped;
  quote errors are the only failures: every input that closes its
    quotes yields words, so `split` is `Result` for the
    unterminated-quote error alone.

Double-quote escapes (the POSIX `2.2.3 Double-Quotes` set Ruby cites,
a backslash retains its meaning only before one of these): `$`,
backtick, `"`, `\`, and a newline. Every other backslash inside
double quotes is retained literally (`"b\qc"` is the word `b\qc`).
Outside quotes a backslash escapes any following scalar (a trailing
backslash with nothing after it is a literal backslash).

ESCAPING (`shellwords-escape`, `shellwords-join`) is the single-quote
strategy, a deliberate divergence from Ruby's backslash escaping: a
string whose every scalar is SAFE is emitted bare, the empty string
becomes `''`, and any other string is wrapped in single quotes with
each internal `'` written `'\''`. THE SAFE SET is exactly ASCII
letters `A-Z`/`a-z`, digits `0-9`, and the eight scalars
`_ - . , : + / @`. Every other scalar (space, tab, newline, `'`,
`"`, `\`, `$`, backtick, `*`, `|`, `>`, `#`, and every non-ASCII
scalar) forces the quoted form. Single quoting makes the round trip
hold for newlines without Ruby's special case: a backslash cannot
spell LF (a backslash plus LF is a shell line continuation), while a
single-quoted LF is a literal scalar.

Named refusals (boundaries this module deliberately does not cross):
  shell expansion: no `$var`, backtick command, globbing, or
    redirection; quotes and backslashes are the whole vocabulary
    (Ruby's own note that this is not a command-line parser);
  NUL: not special here (there is no `exec` boundary in this
    module), so a NUL scalar is escaped like any other unsafe scalar;
  comments: `#` is an ordinary scalar, not a comment introducer;
  backslash escaping: `shellwords-escape` never emits a backslash
    escape (the single-quote strategy above), so its output is meant
    to be used unquoted, exactly like Ruby's.

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes; splitting a command line into words and escaping
words back is that conversion family, so the home is Text.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ShellwordsError">ShellwordsError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ShellwordsError</strong></code></li><li><a class="syn-name" href="#v-render-shellwords-error">render-shellwords-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-shellwords-error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Shellwords/#t-ShellwordsError" title="Text.Shellwords.ShellwordsError">ShellwordsError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-shellwords-escape">shellwords-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shellwords-escape</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-shellwords-join">shellwords-join</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shellwords-join</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-shellwords-split">shellwords-split</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shellwords-split</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Shellwords/#t-ShellwordsError" title="Text.Shellwords.ShellwordsError">ShellwordsError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ShellwordsError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ShellwordsError</strong></code><span class="doc-sig-links"><a href="#t-ShellwordsError" title="Permalink">#</a><a href="/library/source/Text.Shellwords/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

Named errors: the two unterminated-quote shapes Ruby reports as one
`Unmatched quote`; split here so a caller can tell which quote was
left open.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnterminatedSingleQuote"><code><strong class="tok-constructor">UnterminatedSingleQuote</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Shellwords/#t-ShellwordsError" title="Text.Shellwords.ShellwordsError">ShellwordsError</a></code></li><li id="k-UnterminatedDoubleQuote"><code><strong class="tok-constructor">UnterminatedDoubleQuote</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Shellwords/#t-ShellwordsError" title="Text.Shellwords.ShellwordsError">ShellwordsError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-render-shellwords-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-shellwords-error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Shellwords/#t-ShellwordsError" title="Text.Shellwords.ShellwordsError">ShellwordsError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-shellwords-error" title="Permalink">#</a><a href="/library/source/Text.Shellwords/#L84" title="View source at line 84">Source</a></span></div>
</article>

<article class="doc-entry" id="v-shellwords-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shellwords-escape</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-shellwords-escape" title="Permalink">#</a><a href="/library/source/Text.Shellwords/#L193" title="View source at line 193">Source</a></span></div>
</article>

<article class="doc-entry" id="v-shellwords-join">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shellwords-join</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-shellwords-join" title="Permalink">#</a><a href="/library/source/Text.Shellwords/#L218" title="View source at line 218">Source</a></span></div>
<div class="doc-body">

Escape every element and separate with one space (Ruby's
`shelljoin`).

</div>
</article>

<article class="doc-entry" id="v-shellwords-split">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shellwords-split</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Shellwords/#t-ShellwordsError" title="Text.Shellwords.ShellwordsError">ShellwordsError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-shellwords-split" title="Permalink">#</a><a href="/library/source/Text.Shellwords/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

`acc` holds the words emitted so far in reverse; `word` holds the
current word's scalars in reverse; `started` records whether a word
is in progress (set by any field scalar, a backslash, or a quote,
including a quoted empty word, so `''` and `""` survive as empty
scalars).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Shellwords.cinch`](/library/source/Text.Shellwords/). Source SHA-256: `04b6373930f8b453c532abfa2fa2e52461c7308a62e2c1c00c64eb21411bd5f1`.
