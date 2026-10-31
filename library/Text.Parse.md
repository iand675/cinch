# Text.Parse

A source-distributed parser toolkit over the lexer/scanner vocabulary.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Parse`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Parse.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Lex](Text.Lex.md), [Text.Scan](Text.Scan.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Scan](Text.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Parse: parser combinators over the lexed token stream.

Module scheme (design/base-library.md): `Text.*` is text processing
over String/Bytes. This module is the combinator half of the P30
parsing toolkit (`Text.Lex` is the tokenization half): parsers over
the located, laid-out token stream, each parser able to report
source spans for what it consumed, with TYPED errors (the `Parse`
effect's payload) and megaparsec-style error bundles.

The vocabulary follows megaparsec's shape where the bootstrap
allows it:

  * `Parser a` is a state function with the consumed/empty success
    and failure distinction, so `or-else` backtracks exactly when
    megaparsec's `<|>` does and `attempt-p` is the explicit escape;
  * errors are `ParseError` values: `EUnexpected` (a trivial error:
    the offending item plus the expected set) and `EFail` (a fancy
    message), each carrying the source span;
  * `ParseErrorBundle` carries the source name and text with every
    error (the hard failure first, then the recovered ones) and
    renders megaparsec-style with the source line and a caret;
  * `with-recovery-p` is megaparsec's `withRecovery`: a recovery
    handler runs on failure, its recovered error is retained in the
    bundle, and the parse continues from the recovered value.

Typed Parse-effect errors with recovery handlers. `effect Parse` is
the typed error channel (`parse-fail` aborts with a `ParseError`,
`parse-note` records one and resumes). The recovery handlers are
`handler Parse where` clause sets, and `run-parser` /
`run-parser-with` are the worked ones: the runner's failing branch
performs `parse-fail` in its own handled expression and the
recovery handler turns the typed error into the outcome's
megaparsec-style bundle, with `run-parser-with` taking the recovery
handler as an ordinary function on the error value. User code gets
the same shape for its own inline failing computations (`handle
... with (handler Parse where ...)`).

Named refusals (unproductions this module deliberately does not
implement; each is a named boundary, never a silent gap):
  per-call effect dispatch (compiler/CONFORMANCE.md D23): the
    bootstrap dispatches `Parse` operations only from the handled
    computation's own expression, so a COMBINATOR's internal
    failure cannot surface through a caller's `handle`; combinators
    therefore thread typed errors in their replies and only the
    runners above perform the effect. When the bootstrap grows
    per-call dispatch, the combinator failures can move onto the
    effect without changing `ParseError`;
  multi-shot recovery: a recovery handler's value is computed once
    (the bootstrap's one-shot resumption discipline), matching
    megaparsec's `withRecovery` contract;
  incremental lexing: `run-parser` lexes and lays out the whole
    source first (`Text.Lex`), so a lex failure is one bundle error,
    not a resumable stream.

</details>

# Types and constructors

<a id="type-4572726f724974656d"></a>

## `ErrorItem`

```cinch
data ErrorItem
```

Errors and megaparsec-style error bundles.

The item vocabulary of megaparsec's `ErrorItem`: a concrete
token, a named label, or the end of input.

[Source](/reference-sources/Text.Parse.cinch) · line 91

```cinch
ITok :: (Token -> ErrorItem)
```

```cinch
ILabel :: (String -> ErrorItem)
```

```cinch
IEnd :: ErrorItem
```

<a id="type-505374617465"></a>

## `PState`

```cinch
data PState
```

Parser state, replies, the parser type.

Parser state: the remaining located tokens, how many tokens were
consumed (the span combinator's exactness), the span of the last
consumed token, and the recovered (soft) errors so far.

[Source](/reference-sources/Text.Parse.cinch) · line 281

```cinch
PState :: (List LToken -> (Nat -> (Span -> (List ParseError -> PState))))
```

Parser state, replies, the parser type.

Parser state: the remaining located tokens, how many tokens were
consumed (the span combinator's exactness), the span of the last
consumed token, and the recovered (soft) errors so far.

<a id="type-50617273654572726f72"></a>

## `ParseError`

```cinch
data ParseError
```

A typed parse error: megaparsec's trivial error (the offending
item and the expected set) or its fancy `MessageFail`, each with
the source span the error stands at.

[Source](/reference-sources/Text.Parse.cinch) · line 99

```cinch
EUnexpected :: (Span -> (Option ErrorItem -> (List ErrorItem -> ParseError)))
```

```cinch
EFail :: (Span -> (String -> ParseError))
```

<a id="type-50617273654572726f7242756e646c65"></a>

## `ParseErrorBundle`

```cinch
data ParseErrorBundle
```

The bundle: the source name and text every error's positions
index into, and the errors themselves (the hard failure first,
then the recovered ones in occurrence order).

[Source](/reference-sources/Text.Parse.cinch) · line 118

```cinch
Bundle :: (String -> (Bytes -> (List ParseError -> ParseErrorBundle)))
```

<a id="type-50617273654f7574636f6d65"></a>

## `ParseOutcome`

```cinch
data ParseOutcome (a :: Type)
```

Runners: typed Parse-effect errors with recovery handlers.

The parse outcome: the value with its recovered errors, or the
megaparsec-style bundle of the hard failure plus recovered ones.

[Source](/reference-sources/Text.Parse.cinch) · line 570

```cinch
Parsed :: forall a . (a -> (List ParseError -> ParseOutcome a))
```

```cinch
Failed :: forall a . (ParseErrorBundle -> ParseOutcome a)
```

<a id="type-506172736572"></a>

## `Parser`

```cinch
data Parser (a :: Type)
```

A parser: a function from state to reply (megaparsec's `ParsecT`
spine specialized to the pure token stream).

[Source](/reference-sources/Text.Parse.cinch) · line 309

```cinch
MkParser :: forall a . ((PState -> Reply a) -> Parser a)
```

<a id="type-5265706c79"></a>

## `Reply`

```cinch
data Reply (a :: Type)
```

A reply: megaparsec's consumed/empty success and failure
distinction. `ROk` carries the expected-set hint for merging;
`RErr` carries the error and the recovered errors so far.

[Source](/reference-sources/Text.Parse.cinch) · line 303

```cinch
ROk :: forall a . (Bool -> (a -> (PState -> (List ErrorItem -> Reply a))))
```

```cinch
RErr :: forall a . (Bool -> (ParseError -> (List ParseError -> Reply a)))
```

# Values

<a id="value-617474656d70742d70"></a>

## `attempt-p`

```cinch
attempt-p :: forall a . (Parser a -> Parser a)
```

Backtracking (megaparsec's `try`): a failure becomes an
empty-failure so `or-else` may try the alternative.

[Source](/reference-sources/Text.Parse.cinch) · line 340

<a id="value-6265747765656e2d70"></a>

## `between-p`

```cinch
between-p :: forall a open close . (Parser open -> (Parser close -> (Parser a -> Parser a)))
```

[Source](/reference-sources/Text.Parse.cinch) · line 517

<a id="value-62756e646c652d6572726f7273"></a>

## `bundle-errors`

```cinch
bundle-errors :: (ParseErrorBundle -> List ParseError)
```

[Source](/reference-sources/Text.Parse.cinch) · line 129

<a id="value-62756e646c652d6e616d65"></a>

## `bundle-name`

```cinch
bundle-name :: (ParseErrorBundle -> String)
```

[Source](/reference-sources/Text.Parse.cinch) · line 121

<a id="value-62756e646c652d736f75726365"></a>

## `bundle-source`

```cinch
bundle-source :: (ParseErrorBundle -> Bytes)
```

[Source](/reference-sources/Text.Parse.cinch) · line 125

<a id="value-63686f6963652d70"></a>

## `choice-p`

```cinch
choice-p :: forall a . (List (Parser a) -> Parser a)
```

[Source](/reference-sources/Text.Parse.cinch) · line 357

<a id="value-6572726f722d6d65737361676573"></a>

## `error-messages`

```cinch
error-messages :: (ParseError -> List ErrorItem)
```

The expected-item set (empty for `EFail`): the `expecting` line of
the rendered bundle.

[Source](/reference-sources/Text.Parse.cinch) · line 110

<a id="value-6572726f722d7370616e"></a>

## `error-span`

```cinch
error-span :: (ParseError -> Span)
```

[Source](/reference-sources/Text.Parse.cinch) · line 103

<a id="value-6578706563742d656f66"></a>

## `expect-eof`

```cinch
expect-eof :: Parser Unit
```

The end-of-input guard (megaparsec's `eof`): succeeds WITHOUT
consuming when the input is at end (the stream is empty or headed
by the lex pass's `TEOF` marker), and fails empty otherwise. A
guard, never a consumer: it is idempotent (running it twice
succeeds twice) and stands alone in a user parser even though
`run-parser-pure` appends its own whole-input `expect-eof` (the
API finding recorded at D307: the old token-consuming form ran
twice under the runner and the second run failed).

[Source](/reference-sources/Text.Parse.cinch) · line 455

<a id="value-6578706563742d6f70"></a>

## `expect-op`

```cinch
expect-op :: (String -> Parser String)
```

[Source](/reference-sources/Text.Parse.cinch) · line 412

<a id="value-6578706563742d70756e6374"></a>

## `expect-punct`

```cinch
expect-punct :: (String -> Parser String)
```

[Source](/reference-sources/Text.Parse.cinch) · line 407

<a id="value-6578706563742d746f6b"></a>

## `expect-tok`

```cinch
expect-tok :: (Token -> Parser Token)
```

[Source](/reference-sources/Text.Parse.cinch) · line 399

<a id="value-6578706563742d776f7264"></a>

## `expect-word`

```cinch
expect-word :: (String -> Parser String)
```

[Source](/reference-sources/Text.Parse.cinch) · line 402

<a id="value-6661696c2d70"></a>

## `fail-p`

```cinch
fail-p :: forall a . (String -> Parser a)
```

The explicit failure (megaparsec's `fail`): a fancy error at the
current position.

[Source](/reference-sources/Text.Parse.cinch) · line 467

<a id="value-6c6162656c2d70"></a>

## `label-p`

```cinch
label-p :: forall a . (String -> (Parser a -> Parser a))
```

Naming a parser's expectation (megaparsec's `label`): an
empty-failure's expected set becomes the label.

[Source](/reference-sources/Text.Parse.cinch) · line 366

<a id="value-6c6f6f6b2d61686561642d70"></a>

## `look-ahead-p`

```cinch
look-ahead-p :: forall a . (Parser a -> Parser a)
```

Look-ahead: parse without consuming.

[Source](/reference-sources/Text.Parse.cinch) · line 472

<a id="value-6d616e792d70"></a>

## `many-p`

```cinch
many-p :: forall a . (Parser a -> Parser (List a))
```

Repetition and separators. `many-p` never fails; both stop at the
first empty-failure and propagate consumed failures.

[Source](/reference-sources/Text.Parse.cinch) · line 493

<a id="value-6e6f742d666f6c6c6f7765642d62792d70"></a>

## `not-followed-by-p`

```cinch
not-followed-by-p :: forall a . (Parser a -> Parser Unit)
```

megaparsec's `notFollowedBy`: succeed (consuming nothing) only
when the parser fails.

[Source](/reference-sources/Text.Parse.cinch) · line 479

<a id="value-6f7074696f6e616c2d70"></a>

## `optional-p`

```cinch
optional-p :: forall a . (Parser a -> Parser (Option a))
```

[Source](/reference-sources/Text.Parse.cinch) · line 514

<a id="value-6f722d656c7365"></a>

## `or-else`

```cinch
or-else :: forall a . (Parser a -> (Parser a -> Parser a))
```

Choice (megaparsec's `<|>`): the right parser runs only when the
left failed without consuming; two empty failures merge.

[Source](/reference-sources/Text.Parse.cinch) · line 348

<a id="value-70617273652d62696e64"></a>

## `parse-bind`

```cinch
parse-bind :: forall a b . (Parser a -> ((a -> Parser b) -> Parser b))
```

Sequencing (megaparsec's `>>=`): consumed flags combine, and the
expected hints merge only when the second parser consumed
nothing.

[Source](/reference-sources/Text.Parse.cinch) · line 326

<a id="value-70617273652d666d6170"></a>

## `parse-fmap`

```cinch
parse-fmap :: forall a b . ((a -> b) -> (Parser a -> Parser b))
```

[Source](/reference-sources/Text.Parse.cinch) · line 335

<a id="value-70617273652d70757265"></a>

## `parse-pure`

```cinch
parse-pure :: forall a . (a -> Parser a)
```

The combinator core.

[Source](/reference-sources/Text.Parse.cinch) · line 320

<a id="value-7073746174652d636f756e74"></a>

## `pstate-count`

```cinch
pstate-count :: (PState -> Nat)
```

[Source](/reference-sources/Text.Parse.cinch) · line 288

<a id="value-7073746174652d6c617374"></a>

## `pstate-last`

```cinch
pstate-last :: (PState -> Span)
```

[Source](/reference-sources/Text.Parse.cinch) · line 292

<a id="value-7073746174652d736f6674"></a>

## `pstate-soft`

```cinch
pstate-soft :: (PState -> List ParseError)
```

[Source](/reference-sources/Text.Parse.cinch) · line 296

<a id="value-7073746174652d746f6b656e73"></a>

## `pstate-tokens`

```cinch
pstate-tokens :: (PState -> List LToken)
```

[Source](/reference-sources/Text.Parse.cinch) · line 284

<a id="value-72656e6465722d62756e646c65"></a>

## `render-bundle`

```cinch
render-bundle :: (ParseErrorBundle -> String)
```

The bundle's rendering: every error, megaparsec-style, in order.

[Source](/reference-sources/Text.Parse.cinch) · line 234

<a id="value-72656e6465722d6974656d"></a>

## `render-item`

```cinch
render-item :: (ErrorItem -> String)
```

[Source](/reference-sources/Text.Parse.cinch) · line 133

<a id="value-72756e2d706172736572"></a>

## `run-parser`

```cinch
run-parser :: forall a . (String -> (Bytes -> (Parser a -> ParseOutcome a)))
```

The runner with the identity recovery handler: the bundle carries
the typed errors exactly as the parser reported them.

[Source](/reference-sources/Text.Parse.cinch) · line 618

<a id="value-72756e2d7061727365722d666e"></a>

## `run-parser-fn`

```cinch
run-parser-fn :: forall a . (Parser a -> (PState -> Reply a))
```

[Source](/reference-sources/Text.Parse.cinch) · line 312

<a id="value-72756e2d7061727365722d77697468"></a>

## `run-parser-with`

```cinch
run-parser-with :: forall a . (String -> (Bytes -> ((ParseError -> ParseError) -> (Parser a -> ParseOutcome a))))
```

The recovery handler as an ordinary function on the typed error
(the megaparsec-style bundle's final shape is the handler's
choice). The failing path performs `parse-fail` in the handled
computation's own expression and the `handler Parse where` clause
set recovers the typed error into the bundle: typed Parse-effect
errors with recovery handlers, within the bootstrap's lexical
dispatch (named refusal above, decision D23). The hard error
passes through `recover`; the recovered errors of the run stay in
the bundle.

[Source](/reference-sources/Text.Parse.cinch) · line 594

<a id="value-736174697366792d63686172"></a>

## `satisfy-char`

```cinch
satisfy-char :: Parser Nat
```

[Source](/reference-sources/Text.Parse.cinch) · line 442

<a id="value-736174697366792d636f6e"></a>

## `satisfy-con`

```cinch
satisfy-con :: Parser String
```

[Source](/reference-sources/Text.Parse.cinch) · line 422

<a id="value-736174697366792d666c6f6174"></a>

## `satisfy-float`

```cinch
satisfy-float :: Parser String
```

[Source](/reference-sources/Text.Parse.cinch) · line 432

<a id="value-736174697366792d696e74"></a>

## `satisfy-int`

```cinch
satisfy-int :: Parser String
```

[Source](/reference-sources/Text.Parse.cinch) · line 427

<a id="value-736174697366792d737472"></a>

## `satisfy-str`

```cinch
satisfy-str :: Parser Bytes
```

[Source](/reference-sources/Text.Parse.cinch) · line 437

<a id="value-736174697366792d766172"></a>

## `satisfy-var`

```cinch
satisfy-var :: Parser String
```

[Source](/reference-sources/Text.Parse.cinch) · line 417

<a id="value-7365702d62792d70"></a>

## `sep-by-p`

```cinch
sep-by-p :: forall a s . (Parser a -> (Parser s -> Parser (List a)))
```

[Source](/reference-sources/Text.Parse.cinch) · line 507

<a id="value-736f6d652d70"></a>

## `some-p`

```cinch
some-p :: forall a . (Parser a -> Parser (List a))
```

[Source](/reference-sources/Text.Parse.cinch) · line 504

<a id="value-7370616e6e65642d70"></a>

## `spanned-p`

```cinch
spanned-p :: forall a . (Parser a -> Parser (Span, a))
```

Source spans (the toolkit's span story): the consumed tokens'
span, from the first consumed token's start to the last
consumed token's end; a parse that consumes nothing reports a
zero-width span at the current token.

[Source](/reference-sources/Text.Parse.cinch) · line 525

<a id="value-746f6b656e2d70"></a>

## `token-p`

```cinch
token-p :: forall a . ((Token -> Option a) -> (List ErrorItem -> Parser a))
```

The token matcher primitive: consume the head token when the
predicate decodes it, otherwise an empty failure carrying the
offending item and the expected set.

[Source](/reference-sources/Text.Parse.cinch) · line 381

<a id="value-776974682d7265636f766572792d70"></a>

## `with-recovery-p`

```cinch
with-recovery-p :: forall a . ((ParseError -> Parser a) -> (Parser a -> Parser a))
```

Parser-level recovery (megaparsec's `withRecovery`): on failure
the recovery handler runs at the failed parser's start state and
may consume input to resynchronize; its success keeps the
recovered error in the bundle's soft list and the parse
continues. If the recovery handler fails too, the original error
stands (the handler is tried once: one-shot recovery).

[Source](/reference-sources/Text.Parse.cinch) · line 547

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Parse.cinch`. Source SHA-256: `4b6d00ab84e6969cf6fe8a21f29a255a9871c048b6df4baab006734efd7fabed`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
