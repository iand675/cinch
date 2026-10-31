# Text.Json

JSON parsing and rendering over Data.Json, with explicit errors and builder-based output.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Json`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Json.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Builder](Text.Builder.md), [Text.Codec](Text.Codec.md), [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Builder](Text.Builder.md), [Text.Json](Text.Json.md), [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Scan](Text.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Json: JSON text to `Json` values and back, canonically.

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes. This module is the P31 JSON family over the P30
parsing toolkit's error vocabulary and the `Json` value family
(decision D148): a real JSON parser (RFC 8259 section 2's grammar)
with megaparsec-style error bundles, a total printer, the canonical
form (sorted object keys, fixed number formatting, no insignificant
whitespace) for manifests and `.cinchif` round trips, a
SHA-256-strength deterministic content hash, and `JsonCodec`
instances in `Text.Codec`'s law shape over the A6 schema vocabulary.

The parser is built ON the P30 toolkit: the combinator vocabulary
(`parse-pure`, `parse-bind`, `attempt-p`, `or-else`, `label-p`,
`many-p`, `sep-by-p`, `choice-p`, `expect-eof`, ...) is the same
megaparsec shape as `Text.Parse`, carrying the toolkit's own error
types (`ParseError`, `ErrorItem`, `ParseErrorBundle`) and span model,
so `render-bundle` renders JSON errors exactly as it renders grammar
errors. Where the two modules export the same name they are distinct
types (`Text.Parse.Parser` over tokens, `Text.Json.JParser` over the
character stream); import renames or qualification (`import Text.Json
as Json`, `Json.or-else`) disambiguate at use (P19/D108).

WHY THE CHARACTER STREAM AND NOT `Text.Parse`'S TOKEN STREAM
(reported): JSON text cannot be lexed by `Text.Lex` first. The two
escape vocabularies are mutually exclusive in both directions (JSON
`\u0041` is not a Cinch escape, Cinch `\u{41}` is not a JSON
escape; JSON `"\/"` and `"\b"`/`"\f"` are unknown Cinch escapes),
and `Text.Lex` folds numeric spellings (hex folded, underscores
dropped, leading zeros stripped), so `007` and `0x1f` could not be
refused as the JSON grammar demands. A JSON tokenizer feeding
`Text.Parse`'s literal `Parser` would also need `Text.Lex` to export
its `LTok` constructor (module-private after D308's ctor rule); the
combinator shape is carried here over the raw bytes instead, which
keeps every grammar rule honest. When `LTok` is exported this
module's scanners can move onto `List LToken` without API change.

THE CANONICAL FORM (reported): object keys sorted by `string-compare`
(byte order), duplicate keys refused at parse (RFC 8785 requires
unique names; canonical identity needs it), no insignificant
whitespace, integers as canonical decimal (`int-to-string`), floats
as the shortest round-trip scientific text (`textF64ToDecimal`,
`-d.dddde[-]d+`, so `2.5` renders `2.5e0` and `0.5` renders `5e-1`),
strings with exactly the `\"`, `\\`, `\b`, `\f`, `\n`, `\r`, `\t` and
`\u00xx` escapes (other scalars raw UTF-8). Two prints of one value
are byte identical, and the canonical form is a fixed point of
parse-then-print. A non-finite `JFloat` renders `null` (aeson's
choice; JSON numbers are finite), the one rendering that is not a
round trip: the law covers the JSON-representable values.

Total printing (reported): the `Json` family documents the object
and array spines (`JObj`/`JArr` ended by `JObjEnd`/`JArrEnd`) but
cannot enforce them, so a spine tail outside its own family (for
example `JArr x JNull`) renders as one trailing item (arrays) or one
trailing entry under the empty key (objects). Parsing only produces
well-formed spines.

Named refusals (unproductions this module deliberately does not
implement; each is a named boundary, never a silent gap):
  parse recovery and soft notes (`parse-note`, `with-recovery-p`):
    JSON documents report their first error as the bundle; the
    toolkit's recovery vocabulary composes on top when a caller
    wants it;
  comments, `NaN`/`Infinity`, trailing commas and unquoted keys
    (JSON5/JSONC extensions): not RFC 8259;
  lone surrogate escapes: refused with a named error (RFC 8259
    leaves them unpredictable; UTF-8 output cannot carry them);
  streaming/incremental parsing: the document is parsed whole
    (the P30 runner's shape).

</details>

# Types and constructors

<a id="type-4a506172736572"></a>

## `JParser`

```cinch
data JParser (a :: Type)
```

A parser: a function from cursor to reply (the P30 `Parser`
shape). Recursive parsers wrap in `MkJParser` so their references
sit under a lambda (the D302 top-level value discipline).

[Source](/reference-sources/Text.Json.cinch) · line 194

```cinch
MkJParser :: forall a . ((JState -> JReply a) -> JParser a)
```

<a id="type-4a5265706c79"></a>

## `JReply`

```cinch
data JReply (a :: Type)
```

A reply: the consumed/empty success and failure distinction of
`Text.Parse`'s `Reply` (the expected-set hint merges exactly as
there).

[Source](/reference-sources/Text.Json.cinch) · line 187

```cinch
JOk :: forall a . (Bool -> (a -> (JState -> (List ErrorItem -> JReply a))))
```

```cinch
JErr :: forall a . (Bool -> (ParseError -> JReply a))
```

<a id="type-4a5374617465"></a>

## `JState`

```cinch
data JState
```

Parser state, replies, the parser type (the P30 shape over the
character stream: `Text.Parse`'s `PState`/`Reply`/`Parser` carry
tokens, these carry the source cursor).

The cursor into the source: the buffer, the byte offset, and the
position as line and character column (a UTF-8 continuation byte
does not advance the column, so columns count scalars like the
compiler's spans).

[Source](/reference-sources/Text.Json.cinch) · line 165

```cinch
JState :: (Bytes -> (Nat -> (Nat -> (Nat -> JState))))
```

Parser state, replies, the parser type (the P30 shape over the
character stream: `Text.Parse`'s `PState`/`Reply`/`Parser` carry
tokens, these carry the source cursor).

The cursor into the source: the buffer, the byte offset, and the
position as line and character column (a UTF-8 continuation byte
does not advance the column, so columns count scalars like the
compiler's spans).

# Values

<a id="value-617474656d70742d70"></a>

## `attempt-p`

```cinch
attempt-p :: forall a . (JParser a -> JParser a)
```

Backtracking (megaparsec's `try`).

[Source](/reference-sources/Text.Json.cinch) · line 222

<a id="value-6265747765656e2d70"></a>

## `between-p`

```cinch
between-p :: forall a open close . (JParser open -> (JParser close -> (JParser a -> JParser a)))
```

[Source](/reference-sources/Text.Json.cinch) · line 296

<a id="value-63616e6f6e6963616c2d6279746573"></a>

## `canonical-bytes`

```cinch
canonical-bytes :: (Json -> Bytes)
```

[Source](/reference-sources/Text.Json.cinch) · line 829

<a id="value-63616e6f6e6963616c2d68617368"></a>

## `canonical-hash`

```cinch
canonical-hash :: (Json -> String)
```

The content identity of a value: SHA-256 over its canonical bytes
(the hex digest).

[Source](/reference-sources/Text.Json.cinch) · line 834

<a id="value-63616e6f6e6963616c2d6a736f6e"></a>

## `canonical-json`

```cinch
canonical-json :: (Json -> String)
```

The canonical form (the header's rules): object keys sorted.

[Source](/reference-sources/Text.Json.cinch) · line 826

<a id="value-63686f6963652d70"></a>

## `choice-p`

```cinch
choice-p :: forall a . (List (JParser a) -> JParser a)
```

[Source](/reference-sources/Text.Json.cinch) · line 239

<a id="value-6469676573742d686578"></a>

## `digest-hex`

```cinch
digest-hex :: (Bytes -> String)
```

The lowercase hex rendering of a digest: one buffer of two ASCII
digits per byte, filled in place.

[Source](/reference-sources/Text.Json.cinch) · line 121

<a id="value-6578706563742d656f66"></a>

## `expect-eof`

```cinch
expect-eof :: JParser Unit
```

The end of input (the runner appends it: whole-input enforcement
is the default, as in `Text.Parse`'s runner).

[Source](/reference-sources/Text.Json.cinch) · line 302

<a id="value-6661696c2d70"></a>

## `fail-p`

```cinch
fail-p :: forall a . (String -> JParser a)
```

The explicit failure (megaparsec's `fail`): a fancy error at the
current position.

[Source](/reference-sources/Text.Json.cinch) · line 261

<a id="value-6a736f6e2d6571"></a>

## `json-eq`

```cinch
json-eq :: (Json -> (Json -> Bool))
```

Value equality (structural; object entry order is significant
here, the canonical form is the order-free identity).

[Source](/reference-sources/Text.Json.cinch) · line 945

<a id="value-6a736f6e2d76616c7565"></a>

## `json-value`

```cinch
json-value :: JParser Json
```

`json-value` dispatches on the first byte (an empty failure at
anything unexpected, so `or-else` over fragments composes); each
arm consumes its own syntax with consumed-failure errors.

[Source](/reference-sources/Text.Json.cinch) · line 420

<a id="value-6c6162656c2d70"></a>

## `label-p`

```cinch
label-p :: forall a . (String -> (JParser a -> JParser a))
```

Naming a parser's expectation (megaparsec's `label`).

[Source](/reference-sources/Text.Json.cinch) · line 247

<a id="value-6c6f6f6b2d61686561642d70"></a>

## `look-ahead-p`

```cinch
look-ahead-p :: forall a . (JParser a -> JParser a)
```

Look-ahead: parse without consuming.

[Source](/reference-sources/Text.Json.cinch) · line 265

<a id="value-6d616e792d70"></a>

## `many-p`

```cinch
many-p :: forall a . (JParser a -> JParser (List a))
```

Repetition and separators (`many-p` never fails; both stop at the
first empty failure and propagate consumed failures).

[Source](/reference-sources/Text.Json.cinch) · line 272

<a id="value-6f7074696f6e616c2d70"></a>

## `optional-p`

```cinch
optional-p :: forall a . (JParser a -> JParser (Option a))
```

[Source](/reference-sources/Text.Json.cinch) · line 293

<a id="value-6f722d656c7365"></a>

## `or-else`

```cinch
or-else :: forall a . (JParser a -> (JParser a -> JParser a))
```

Choice (megaparsec's `<|>`): the right parser runs only when the
left failed without consuming; two empty failures merge.

[Source](/reference-sources/Text.Json.cinch) · line 230

<a id="value-70617273652d62696e64"></a>

## `parse-bind`

```cinch
parse-bind :: forall a b . (JParser a -> ((a -> JParser b) -> JParser b))
```

[Source](/reference-sources/Text.Json.cinch) · line 209

<a id="value-70617273652d666d6170"></a>

## `parse-fmap`

```cinch
parse-fmap :: forall a b . ((a -> b) -> (JParser a -> JParser b))
```

[Source](/reference-sources/Text.Json.cinch) · line 218

<a id="value-70617273652d6a736f6e"></a>

## `parse-json`

```cinch
parse-json :: (String -> (Bytes -> ParseOutcome Json))
```

The document runner: the toolkit's `ParseOutcome`, so
`Text.Parse.render-bundle` renders the failure megaparsec-style.

[Source](/reference-sources/Text.Json.cinch) · line 800

<a id="value-70617273652d70757265"></a>

## `parse-pure`

```cinch
parse-pure :: forall a . (a -> JParser a)
```

The combinator core (the P30 vocabulary; semantics follow
`Text.Parse` exactly).

[Source](/reference-sources/Text.Json.cinch) · line 206

<a id="value-7072696e742d6a736f6e"></a>

## `print-json`

```cinch
print-json :: (Json -> String)
```

Printing (total), canonicalization, content identity.

The printers emit into a `Builder` and flatten once: concatenating
each rendered piece onto the rendering of the rest re-copies that
whole suffix at every step, quadratic in the document's length.

The total printer: object entries in spine order, no whitespace.

[Source](/reference-sources/Text.Json.cinch) · line 822

<a id="value-72756e2d6a7061727365722d666e"></a>

## `run-jparser-fn`

```cinch
run-jparser-fn :: forall a . (JParser a -> (JState -> JReply a))
```

[Source](/reference-sources/Text.Json.cinch) · line 197

<a id="value-736174697366792d62797465"></a>

## `satisfy-byte`

```cinch
satisfy-byte :: forall a . ((Nat -> Option a) -> (List ErrorItem -> JParser a))
```

The cursor primitive (the character stream's `token-p`): consume
one byte when the predicate decodes it.

[Source](/reference-sources/Text.Json.cinch) · line 309

<a id="value-7365702d62792d70"></a>

## `sep-by-p`

```cinch
sep-by-p :: forall a s . (JParser a -> (JParser s -> JParser (List a)))
```

[Source](/reference-sources/Text.Json.cinch) · line 286

<a id="value-736f6d652d70"></a>

## `some-p`

```cinch
some-p :: forall a . (JParser a -> JParser (List a))
```

[Source](/reference-sources/Text.Json.cinch) · line 283

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Text.Json.JsonCodec_Json` | `JsonCodec Json` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Json.cinch`. Source SHA-256: `a80f141038155fcd58bf92ae57204f0b8c0b098c9144a90fced2f37606782876`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
