# Text.Shellwords

Shell-style word splitting/joining; this is data parsing, not execution of a shell command.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Shellwords`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Shellwords.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Scan](Text.Scan.md).

Non-embedded source closure: [Text.Scan](Text.Scan.md), [Text.Shellwords](Text.Shellwords.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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
    hostile strings in compiler/tests/lib_shellwords.rs);
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

# Types and constructors

<a id="type-5368656c6c776f7264734572726f72"></a>

## `ShellwordsError`

```cinch
data ShellwordsError
```

Named errors: the two unterminated-quote shapes Ruby reports as one
`Unmatched quote`; split here so a caller can tell which quote was
left open.

[Source](/reference-sources/Text.Shellwords.cinch) · line 80

```cinch
UnterminatedSingleQuote :: ShellwordsError
```

```cinch
UnterminatedDoubleQuote :: ShellwordsError
```

# Values

<a id="value-72656e6465722d7368656c6c776f7264732d6572726f72"></a>

## `render-shellwords-error`

```cinch
render-shellwords-error :: (ShellwordsError -> String)
```

[Source](/reference-sources/Text.Shellwords.cinch) · line 84

<a id="value-7368656c6c776f7264732d657363617065"></a>

## `shellwords-escape`

```cinch
shellwords-escape :: (String -> String)
```

[Source](/reference-sources/Text.Shellwords.cinch) · line 193

<a id="value-7368656c6c776f7264732d6a6f696e"></a>

## `shellwords-join`

```cinch
shellwords-join :: (List String -> String)
```

Escape every element and separate with one space (Ruby's
`shelljoin`).

[Source](/reference-sources/Text.Shellwords.cinch) · line 218

<a id="value-7368656c6c776f7264732d73706c6974"></a>

## `shellwords-split`

```cinch
shellwords-split :: (String -> Result ShellwordsError (List String))
```

Splitting (Ruby Shellwords.split).

`acc` holds the words emitted so far in reverse; `word` holds the
current word's scalars in reverse; `started` records whether a word
is in progress (set by any field scalar, a backslash, or a quote,
including a quoted empty word, so `''` and `""` survive as empty
scalars).

[Source](/reference-sources/Text.Shellwords.cinch) · line 98

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Shellwords.cinch`. Source SHA-256: `7b371e38d5fdfeb48bcd293783859f56e645870a217f605036f5e0d5aaa486a9`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
