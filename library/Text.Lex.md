# Text.Lex

A source-distributed lexer toolkit: tokens, spans, and lexical errors for compiler/parser clients.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Lex`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Lex.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Lex](Text.Lex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Lex: token-level lexing and the layout pass.

Module scheme (design/base-library.md): `Text.*` is text processing
over String/Bytes. This module is the tokenization half of the P30
parsing toolkit (`Text.Parse` is the combinator half): it turns
UTF-8 source bytes into the located token stream the surface
grammar's EBNF consumes, WITH the deterministic layout pass already
applied ("the EBNF consumes explicit `{`, `;` and `}` tokens after a
deterministic layout pass", surface syntax and grammar, Layout and
lexing).

No drift from the layout spec is the design rule here. Every rule
this scanner implements cites the sentence it follows, and the
behavior is pinned token for token against the bootstrap compiler's
own lexer and layout pass (`compiler/src/lexer.rs`,
`compiler/src/layout.rs`) in `compiler/tests/parsers.rs`: the
toolkit's token stream must agree with the compiler's on the shared
corpus (kind, spelling, line, column, virtual flag), so a spec
change that reaches the compiler cannot silently leave this module
behind. Where the two could disagree, this module follows the
compiler.

The lexical classes follow "Layout and lexing" and the local
amendments: identifiers with dash glue where a `-` is tight on both
sides between identifier characters, digits not continuing a name
(amendment section 8); a `-` tight only to what follows signing a
negative literal unless the previous character ends an expression
(amendment section 8); reserved punctuation matched longest first
(`::`, `->`, `<-`, `=>`, `[|`, `|]`, `$(`, the multiplicities, and
the bare reserved symbols `!`, `@`, `.`, `..`, `=`, `|`, `$`, `\`,
`~`); comment recognition taking priority over symbol identifiers;
`$` continuing an operator run (`<$>`) but never starting one;
strings supporting `\\`, `\"`, `\'`, `\n`, `\r`, `\t`, `\u{HEX}`
with a valid scalar; tabs rejected in indentation; line endings
normalizing to LF.

The self-hosting forms the compiler's own sources use are lexed
here, each as a token that carries what a parser needs:
  `#[name]`, `#[name(...)]` and `#[name=...]` attribute annotations
    (amendment section 5) become one `TAttr` token holding the inner
    text between `#[` and `]`;
  `prim` bodies (primitive operations and inline target IR
    specification section 2) become the `prim` keyword plus the
    balanced `{ ... }` region a registered family name opens, as
    the compiler emits it: `TPunct "{"`, one `TPrimFrag` token
    holding the verbatim inner IR text, and `TPunct "}"`;
  the tick name forms `'name` and `''T` (amendment section 12)
    become `TTickName` and `TTickType` tokens holding the name
    (only the `Char` literal form `'c'` stays a `TChar`).

Named refusals (malformed input each is an error naming itself,
never a silent drift): `RefusedAttr` for an attribute with no valid
name or no closing `]`, `RefusedPrim` for an unterminated `prim`
body fragment, and `RefusedTickName` for a tick that is neither a
`Char` literal nor a name. All three kinds stay exported names (a
module that mentions one keeps resolving, and `Text.Parse`'s error
vocabulary is unchanged); what moved is which inputs reach them: the
three well-formed forms above once refused and now lex to the
carrying tokens, so a refusal means malformed input, not a form
this module has not learned.

</details>

# Types and constructors

<a id="type-4c546f6b656e"></a>

## `LToken`

```cinch
data LToken
```

A token with its span; the flag marks `{`, `;`, `}` inserted by
the layout pass (the compiler's `Token.virtual_`).

[Source](/reference-sources/Text.Lex.cinch) · line 178

<a id="type-4c65784572726f72"></a>

## `LexError`

```cinch
data LexError
```

[Source](/reference-sources/Text.Lex.cinch) · line 353

```cinch
LexError :: (Span -> (LexErrorKind -> LexError))
```

<a id="type-4c65784572726f724b696e64"></a>

## `LexErrorKind`

```cinch
data LexErrorKind
```

Lex errors.

Every rejection this module can produce. The layout kinds carry
the columns the compiler's diagnostics name; the refusal kinds are
the named unproductions of the module header.

[Source](/reference-sources/Text.Lex.cinch) · line 335

```cinch
TabInIndentation :: LexErrorKind
```

```cinch
UnterminatedComment :: LexErrorKind
```

```cinch
UnterminatedString :: LexErrorKind
```

```cinch
UnterminatedChar :: LexErrorKind
```

```cinch
BadEscape :: LexErrorKind
```

```cinch
MalformedNumber :: LexErrorKind
```

```cinch
UnexpectedChar :: LexErrorKind
```

```cinch
MalformedTick :: LexErrorKind
```

```cinch
LayoutTooShallow :: (Nat -> LexErrorKind)
```

```cinch
MismatchedDelimiter :: LexErrorKind
```

```cinch
UnclosedDelimiter :: LexErrorKind
```

```cinch
InWithoutLet :: LexErrorKind
```

```cinch
InCrossesDelimiter :: LexErrorKind
```

```cinch
RefusedAttr :: LexErrorKind
```

```cinch
RefusedPrim :: LexErrorKind
```

```cinch
RefusedTickName :: LexErrorKind
```

<a id="type-4c65786564"></a>

## `Lexed`

```cinch
data Lexed
```

[Source](/reference-sources/Text.Lex.cinch) · line 385

```cinch
LexedTokens :: (List LToken -> Lexed)
```

```cinch
LexedFail :: (Span -> (LexErrorKind -> Lexed))
```

<a id="type-5370616e"></a>

## `Span`

```cinch
data Span
```

Spans, tokens, located tokens.

A source span: byte offsets [start, end) and the start position as
line and character column (the compiler's `Span` shape).

[Source](/reference-sources/Text.Lex.cinch) · line 124

```cinch
Span :: (Nat -> (Nat -> (Nat -> (Nat -> Span))))
```

Spans, tokens, located tokens.

A source span: byte offsets [start, end) and the start position as
line and character column (the compiler's `Span` shape).

<a id="type-546f6b656e"></a>

## `Token`

```cinch
data Token
```

One token of the surface grammar's tokenization: the lexical
classes of "Layout and lexing" after the local amendments. The
spelling payloads are the canonical forms the compiler's lexer
records: `TInt` is the canonical decimal digit string (hex folded,
underscores dropped, leading zeros stripped, a leading `-` for a
signed literal), `TFloat` the scanned decimal text, `TStr` the
decoded UTF-8 content, `TChar` the decoded scalar.

[Source](/reference-sources/Text.Lex.cinch) · line 150

```cinch
TVar :: (String -> Token)
```

```cinch
TCon :: (String -> Token)
```

```cinch
TOp :: (String -> Token)
```

```cinch
TInt :: (String -> Token)
```

```cinch
TFloat :: (String -> Token)
```

```cinch
TStr :: (Bytes -> Token)
```

```cinch
TChar :: (Nat -> Token)
```

```cinch
TWord :: (String -> Token)
```

```cinch
TPunct :: (String -> Token)
```

```cinch
TAttr :: (String -> Token)
```

One `#[...]` attribute annotation: the inner text between `#[`
and `]` (the compiler's `Attr`, whose name and raw argument text
a parser splits out).

```cinch
TPrimFrag :: (String -> Token)
```

The verbatim inner text of a `prim` body's inline target-IR
fragment (the compiler's `Fragment`): the balanced `{ ... }`
region a registered family name opens, captured raw so fragment
grammars never pass through the surface tokenizer.

```cinch
TTickName :: (String -> Token)
```

`'name`: the tick lexeme's symbol form (amendment section 12),
the name `lex_word`'s shape, dashes included.

```cinch
TTickType :: (String -> Token)
```

`''T`: the tick lexeme's type-name form (amendment section 12).

```cinch
TEOF :: Token
```

# Values

<a id="value-62797465732d736c696365"></a>

## `bytes-slice`

```cinch
bytes-slice :: using (arg2 <= (Data.Bytes.bytes-len arg0)) => ((arg0 :: Bytes) -> ((arg1 :: Nat) -> ((arg2 :: Nat) -> Bytes)))
```

Small text helpers.

`pack` is provided by Data.String (the type home); this module
imports it for its char-list-to-String conversions.

The byte range [i, j) of a buffer as a fresh copy; a reversed range
(`j < i`) holds no bytes, so it is the empty copy. The range's end
must lie inside the buffer: every caller slices between offsets it
has already scanned, so it holds that bound.

[Source](/reference-sources/Text.Lex.cinch) · line 96

<a id="value-63686172732d6279746573"></a>

## `chars-bytes`

```cinch
chars-bytes :: (List Char -> Bytes)
```

[Source](/reference-sources/Text.Lex.cinch) · line 115

<a id="value-65712d746f6b656e"></a>

## `eq-token`

```cinch
eq-token :: (Token -> (Token -> Bool))
```

Token equality over the kind and its spelling: `Text.Parse`'s
token matchers compare with this.

[Source](/reference-sources/Text.Lex.cinch) · line 205

<a id="value-6c61796f75742d746f6b656e73"></a>

## `layout-tokens`

```cinch
layout-tokens :: (List LToken -> Lexed)
```

fields: out, stack, prev, pending-span, pending-is-let, with-paren-pending,
supply-close-pending

[Source](/reference-sources/Text.Lex.cinch) · line 1372

<a id="value-6c65782d6572726f722d6b696e64"></a>

## `lex-error-kind`

```cinch
lex-error-kind :: (LexError -> LexErrorKind)
```

[Source](/reference-sources/Text.Lex.cinch) · line 360

<a id="value-6c65782d6572726f722d6d657373616765"></a>

## `lex-error-message`

```cinch
lex-error-message :: (LexErrorKind -> String)
```

[Source](/reference-sources/Text.Lex.cinch) · line 364

<a id="value-6c65782d6572726f722d7370616e"></a>

## `lex-error-span`

```cinch
lex-error-span :: (LexError -> Span)
```

[Source](/reference-sources/Text.Lex.cinch) · line 356

<a id="value-6c65782d736f75726365"></a>

## `lex-source`

```cinch
lex-source :: (Bytes -> Lexed)
```

lex-tokens over a source `String`'s bytes.

[Source](/reference-sources/Text.Lex.cinch) · line 1290

<a id="value-6c65782d746f6b656e73"></a>

## `lex-tokens`

```cinch
lex-tokens :: (Bytes -> Lexed)
```

[Source](/reference-sources/Text.Lex.cinch) · line 1194

<a id="value-6c742d7370616e"></a>

## `lt-span`

```cinch
lt-span :: (LToken -> Span)
```

[Source](/reference-sources/Text.Lex.cinch) · line 181

<a id="value-6c742d746f6b656e"></a>

## `lt-token`

```cinch
lt-token :: (LToken -> Token)
```

[Source](/reference-sources/Text.Lex.cinch) · line 189

<a id="value-6c742d7669727475616c"></a>

## `lt-virtual`

```cinch
lt-virtual :: (LToken -> Bool)
```

[Source](/reference-sources/Text.Lex.cinch) · line 185

<a id="value-72656e6465722d746f6b656e"></a>

## `render-token`

```cinch
render-token :: (LToken -> String)
```

The canonical rendering the drift tests compare against the
compiler's lexer: `kind:spelling@line:col`, with `!v` marking a
layout-inserted token. A spelling's `\` and newline characters
render as the two-character escapes `\\` and `\n`, so one token
is always one line of the rendering (the drift tests split on
lines); a `TChar` renders its scalar's UTF-8 text (the compiler
records the decoded char).

[Source](/reference-sources/Text.Lex.cinch) · line 291

<a id="value-7370616e2d636f6c"></a>

## `span-col`

```cinch
span-col :: (Span -> Nat)
```

[Source](/reference-sources/Text.Lex.cinch) · line 139

<a id="value-7370616e2d656e64"></a>

## `span-end`

```cinch
span-end :: (Span -> Nat)
```

[Source](/reference-sources/Text.Lex.cinch) · line 131

<a id="value-7370616e2d6c696e65"></a>

## `span-line`

```cinch
span-line :: (Span -> Nat)
```

[Source](/reference-sources/Text.Lex.cinch) · line 135

<a id="value-7370616e2d7374617274"></a>

## `span-start`

```cinch
span-start :: (Span -> Nat)
```

[Source](/reference-sources/Text.Lex.cinch) · line 127

<a id="value-746f6b656e2d6b696e642d6e616d65"></a>

## `token-kind-name`

```cinch
token-kind-name :: (Token -> String)
```

[Source](/reference-sources/Text.Lex.cinch) · line 250

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Lex.cinch`. Source SHA-256: `aac19e1c9e6d49781285e77d5818d2d570a6e2a46f244495324495b71c921998`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
