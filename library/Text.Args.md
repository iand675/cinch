# Text.Args

Command-line argument processing and its named value/error vocabulary.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Args`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Args.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Text.Args](Text.Args.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Args: CLI argument parsing over an argv-shaped `List String`.

Module scheme (design/base-library.md): `Text.*` is conversion to
and from String/Bytes; this is the driver port's argument reader (a
small `Text.Args`-shape module, P31): named options and positional
values out of `argv`, with named errors.

The argv grammar (after the program name):

  `--name`         a flag occurrence (`FlagSpec`)
  `--name value`   a value occurrence (`ValueSpec`)
  `--name=value`   the same, spelled inline
  `--`             the end of options; every later word is a value
  bare word        a positional value

A `Required`-wrapped spec must occur (a missing one is
`MissingRequired`); every other spec is optional. Occurrences keep
argv order (an option may repeat). `render-argv` renders an `Args`
back through the `--name=value` spelling, so the round trip
`parse-args specs (render-argv a) == Ok a` holds for well-formed
`a` (asserted over sample argvs in compiler/tests/json.rs).

Named refusals (unproductions this module deliberately does not
implement; each is a named boundary, never a silent gap):
  short-option clusters (`-abc`) and `-o value` single-dash forms:
    long options only (the Cinch driver spells `--output`);
  option-argument syntax beyond `--name=value` and `--name value`;
  `--name=` with an empty value: the value is the empty string
    (an occurrence, not a refusal).

</details>

# Types and constructors

<a id="type-41726753706563"></a>

## `ArgSpec`

```cinch
data ArgSpec
```

One option's spec: a flag (no value), a value option, or a
required wrapper around either.

[Source](/reference-sources/Text.Args.cinch) · line 52

```cinch
FlagSpec :: (String -> ArgSpec)
```

```cinch
ValueSpec :: (String -> ArgSpec)
```

```cinch
Required :: (ArgSpec -> ArgSpec)
```

<a id="type-41726773"></a>

## `Args`

```cinch
data Args
```

The parse result: the option occurrences in argv order and the
positional values in argv order.

[Source](/reference-sources/Text.Args.cinch) · line 72

```cinch
Args :: (List OptVal -> (List String -> Args))
```

The parse result: the option occurrences in argv order and the
positional values in argv order.

<a id="type-417267734572726f72"></a>

## `ArgsError`

```cinch
data ArgsError
```

The named errors: an option the specs do not know, a value option
without its value, a value on a flag, a required spec that did
not occur.

[Source](/reference-sources/Text.Args.cinch) · line 86

```cinch
UnknownOption :: (String -> ArgsError)
```

```cinch
MissingValue :: (String -> ArgsError)
```

```cinch
UnexpectedValue :: (String -> ArgsError)
```

```cinch
MissingRequired :: (String -> ArgsError)
```

<a id="type-4f707456616c"></a>

## `OptVal`

```cinch
data OptVal
```

One option occurrence: its name and its value (a flag's value is
`None`).

[Source](/reference-sources/Text.Args.cinch) · line 59

```cinch
OptVal :: (String -> (Option String -> OptVal))
```

One option occurrence: its name and its value (a flag's value is
`None`).

# Values

<a id="value-617267732d6f7074696f6e73"></a>

## `args-options`

```cinch
args-options :: (Args -> List OptVal)
```

[Source](/reference-sources/Text.Args.cinch) · line 75

<a id="value-617267732d76616c756573"></a>

## `args-values`

```cinch
args-values :: (Args -> List String)
```

[Source](/reference-sources/Text.Args.cinch) · line 79

<a id="value-6f70742d6e616d65"></a>

## `opt-name`

```cinch
opt-name :: (OptVal -> String)
```

[Source](/reference-sources/Text.Args.cinch) · line 62

<a id="value-6f70742d76616c7565"></a>

## `opt-value`

```cinch
opt-value :: (OptVal -> Option String)
```

[Source](/reference-sources/Text.Args.cinch) · line 66

<a id="value-70617273652d61726773"></a>

## `parse-args`

```cinch
parse-args :: (List ArgSpec -> (List String -> Result ArgsError Args))
```

The argv walk: options and values accumulate in order; `--` ends
the options.

[Source](/reference-sources/Text.Args.cinch) · line 124

<a id="value-72656e6465722d617267732d6572726f72"></a>

## `render-args-error`

```cinch
render-args-error :: (ArgsError -> String)
```

[Source](/reference-sources/Text.Args.cinch) · line 92

<a id="value-72656e6465722d61726776"></a>

## `render-argv`

```cinch
render-argv :: (Args -> List String)
```

The canonical argv of one parse: `--name=value` per value
occurrence, `--name` per flag, then the positional values.

[Source](/reference-sources/Text.Args.cinch) · line 202

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Args.cinch`. Source SHA-256: `d32463cc68034db08180c0417761348dd772852b1d99a3d24db7d6e3492a91ec`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
