# Text.OptionParser

Declarative option specifications, short/long forms, conversion, positionals, usage, and named parse errors.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.OptionParser`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.OptionParser.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Scan](Text.Scan.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.OptionParser](Text.OptionParser.md), [Text.Scan](Text.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.OptionParser: Ruby's `OptionParser` as a declarative flag
specification over an argv-shaped `List String`.

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes, and this is the argv reader of the `Text.*`
argument family (the Ruby `OptionParser` item of the P31 tooling port,
design/stdlib-parity-ruby.md). One `List OptionSpec` declares the whole
surface (short and long spelling, the named converter a value runs
through, the help text) and `parse-options` turns an argv into typed
bindings, with no per-option callback.

RELATION TO Text.Args. `Text.Args` is the low-level argv walker: it
takes `List ArgSpec` (an option NAME, no converter), returns the
occurrences in argv order as raw `String`s, and its named refusal is
exactly the short-option family (its header refuses `-abc` clusters and
`-o value`), so a short word falls through there as a positional value.
This module is the declarative layer ABOVE that grammar: it accepts a
superset of the same argv forms (adding short names, clusters and
prefix matching), binds each option at its declared type through the
converter, and renders the usage table. It does NOT delegate to
`Text.Args`, because the short forms `Text.Args` refuses are exactly the
forms this module must accept; the two agree on the long `--name` /
`--name=value` / `--name value` words, which the tests assert over one
shared argv.

The argv grammar (after the program name):

  --name          a long flag (no value)
  --name value    a long option and its separate value
  --name=value    the same, `=`-joined
  --abbrev        a unique long-name PREFIX (an exact name wins first)
  -n              a short flag
  -n value        a short option and its separate value
  -nvalue         the same, attached (a leading `=` is accepted:
                  `-n=value`)
  -abc            a cluster of short flags; the cluster breaks at the
                  first option that takes a value, whose value is the
                  rest of the cluster or the next word
  --              the end of options (every later word is a value)
  -               a positional value (the stdin-marker convention)
  bare word       a positional value

Law shape (what the tests assert): a value option binds the named
converter applied to its value spelling (`--count=3` binds the integer
3, and the `=`-joined and separated spellings agree), a binding is keyed
by the LONG name, a repeated option's LAST occurrence wins, positionals
keep argv order, and an argv outside the grammar returns a named error
with no partial result (the parse is total over admitted input, refusing
rather than guessing).

Named refusals (unproductions this module deliberately does not
implement; each is a named boundary, never a silent gap):
  `--[no-]name` negation and any boolean override spelling: a flag's
    presence is a `VBool True` binding, absence is no binding at all;
  per-option callbacks (Ruby's `on` blocks): the result is a value, not
    a handler, so there is no subcommand or `case` dispatch here;
  required options: every spec is optional (a must-occur option is the
    caller's check on `option-value`, or `Text.Args.Required`);
  single-dash LONG names (`-output`): a single dash always opens a short
    cluster, so `-output` reads as the cluster of `o`,`u`,...;
  leading-dash numerics (`-3`): a short-option word
    (`UnknownOption "-3"`), not a negative positional, unless it follows
    `--`;
  response files (`@file`) and environment fallbacks: argv only;
  a value on a flag (`--verbose=1`): `UnexpectedValue`;
  duplicate long names across specs: not diagnosed, the first spec in
    the list wins (the specification is the caller's to keep coherent).

</details>

# Types and constructors

<a id="type-436f6e766572746572"></a>

## `Converter`

```cinch
data Converter
```

The specification.

The value converter family. `ConvFlag` takes no value: its presence
binds `VBool True`, which is the whole of a flag's information.

[Source](/reference-sources/Text.OptionParser.cinch) · line 98

```cinch
ConvString :: Converter
```

```cinch
ConvInt :: Converter
```

```cinch
ConvFloat :: Converter
```

```cinch
ConvFlag :: Converter
```

<a id="type-4f7074696f6e50617273654572726f72"></a>

## `OptionParseError`

```cinch
data OptionParseError
```

The named errors.

One error constructor per refusal: an unknown option (the spelling is
carried verbatim), a value option whose value is missing, a value that
fails its converter (the name and the offending spelling), an
ambiguous long-name prefix (the prefix and its candidates, in spec
order), and a value on a flag.

[Source](/reference-sources/Text.OptionParser.cinch) · line 226

```cinch
UnknownOption :: (String -> OptionParseError)
```

```cinch
MissingArgument :: (String -> OptionParseError)
```

```cinch
InvalidValue :: (String -> (String -> OptionParseError))
```

```cinch
AmbiguousOption :: (String -> (List String -> OptionParseError))
```

```cinch
UnexpectedValue :: (String -> OptionParseError)
```

<a id="type-4f7074696f6e53706563"></a>

## `OptionSpec`

```cinch
data OptionSpec
```

One option's spec: an optional single-character short name, the
required long name (the binding key), the converter, and the help line
for `option-help`. `spec-takes-value` is derived from the converter
(every converter but `ConvFlag` consumes a value).

[Source](/reference-sources/Text.OptionParser.cinch) · line 108

```cinch
OptSpec :: (Option String -> (String -> (Converter -> (String -> OptionSpec))))
```

<a id="type-4f7074696f6e56616c7565"></a>

## `OptionValue`

```cinch
data OptionValue
```

The bound values.

A bound value is one of the four converter images. `VBool` carries a
flag's presence.

[Source](/reference-sources/Text.OptionParser.cinch) · line 141

```cinch
VString :: (String -> OptionValue)
```

```cinch
VInt :: (Int -> OptionValue)
```

```cinch
VFloat :: (F64 -> OptionValue)
```

```cinch
VBool :: (Bool -> OptionValue)
```

<a id="type-5061727365644f7074696f6e73"></a>

## `ParsedOptions`

```cinch
data ParsedOptions
```

The result and its accessors.

The parse result: the bindings keyed by long name (each name once,
holding its LAST occurrence) and the positional values in argv order.
Flags bind `VBool True`; `option-flags` is the flag subset of that
list.

[Source](/reference-sources/Text.OptionParser.cinch) · line 189

```cinch
ParsedOptions :: (List (String, OptionValue) -> (List String -> ParsedOptions))
```

The result and its accessors.

The parse result: the bindings keyed by long name (each name once,
holding its LAST occurrence) and the positional values in argv order.
Flags bind `VBool True`; `option-flags` is the flag subset of that
list.

# Values

<a id="value-6f7074696f6e2d666c61672d70726573656e74"></a>

## `option-flag-present`

```cinch
option-flag-present :: (ParsedOptions -> (String -> Bool))
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 201

<a id="value-6f7074696f6e2d666c616773"></a>

## `option-flags`

```cinch
option-flags :: (ParsedOptions -> List String)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 208

<a id="value-6f7074696f6e2d686173"></a>

## `option-has`

```cinch
option-has :: (ParsedOptions -> (String -> Bool))
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 196

<a id="value-6f7074696f6e2d68656c70"></a>

## `option-help`

```cinch
option-help :: (List OptionSpec -> String)
```

The usage table.

`option-help` renders one row per spec: the short/long spellings and
the value placeholder in a padded left column, then the help text, so
the rendered table names every declared option.

[Source](/reference-sources/Text.OptionParser.cinch) · line 529

<a id="value-6f7074696f6e2d706f736974696f6e616c73"></a>

## `option-positionals`

```cinch
option-positionals :: (ParsedOptions -> List String)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 212

<a id="value-6f7074696f6e2d76616c7565"></a>

## `option-value`

```cinch
option-value :: (ParsedOptions -> (String -> Option OptionValue))
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 192

<a id="value-70617273652d6f7074696f6e73"></a>

## `parse-options`

```cinch
parse-options :: (List OptionSpec -> (List String -> Result OptionParseError ParsedOptions))
```

The parse.

[Source](/reference-sources/Text.OptionParser.cinch) · line 246

<a id="value-72656e6465722d6f7074696f6e2d70617273652d6572726f72"></a>

## `render-option-parse-error`

```cinch
render-option-parse-error :: (OptionParseError -> String)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 233

<a id="value-737065632d636f6e766572746572"></a>

## `spec-converter`

```cinch
spec-converter :: (OptionSpec -> Converter)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 119

<a id="value-737065632d68656c70"></a>

## `spec-help`

```cinch
spec-help :: (OptionSpec -> String)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 123

<a id="value-737065632d6c6f6e67"></a>

## `spec-long`

```cinch
spec-long :: (OptionSpec -> String)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 115

<a id="value-737065632d73686f7274"></a>

## `spec-short`

```cinch
spec-short :: (OptionSpec -> Option String)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 111

<a id="value-737065632d74616b65732d76616c7565"></a>

## `spec-takes-value`

```cinch
spec-takes-value :: (OptionSpec -> Bool)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 127

<a id="value-76616c75652d626f6f6c"></a>

## `value-bool`

```cinch
value-bool :: (OptionValue -> Option Bool)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 162

<a id="value-76616c75652d666c6f6174"></a>

## `value-float`

```cinch
value-float :: (OptionValue -> Option F64)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 157

<a id="value-76616c75652d696e74"></a>

## `value-int`

```cinch
value-int :: (OptionValue -> Option Int)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 152

<a id="value-76616c75652d737472696e67"></a>

## `value-string`

```cinch
value-string :: (OptionValue -> Option String)
```

[Source](/reference-sources/Text.OptionParser.cinch) · line 147

<a id="value-76616c75652d746f2d737472696e67"></a>

## `value-to-string`

```cinch
value-to-string :: (OptionValue -> String)
```

The rendering (the `Text.*` conversion direction): a value's text for
diagnostics. The float uses the runtime's shortest round-trip decimal
(the same spelling `Data.Machine.f64-to-decimal` gives, which `Json`'s
printer uses).

[Source](/reference-sources/Text.OptionParser.cinch) · line 171

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/OptionParser.cinch`. Source SHA-256: `f6f1d8610648fed5f3355567cdd52a381f80322f8996ce29482931ef0977cc48`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
