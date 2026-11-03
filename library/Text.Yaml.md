# Text.Yaml

The module's supported YAML subset, structured values, and named parse errors; not an assertion of every YAML feature.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Yaml`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Yaml.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Yaml](Text.Yaml.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Yaml: the YAML safe data subset: text to `Json` values and back.

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes, and this module is the P37 serialization family's
YAML member (design/stdlib-parity-ruby.md: Ruby's YAML loads are the
classic arbitrary-object-deserialization CVE, so the port is a
REDESIGN, not a translation). It speaks the JSON family's value
vocabulary (`Data.Json`'s `Json`, decision D148) so the codec family
has one shape: `yaml-parse :: String -> Result YamlError Json` and
`yaml-print :: Json -> String`. Like `Text.Json`/`Text.Args` it is an
ordinary importable module beside the entry, not a base-layer module
(the P30/P31 pattern, D309). Unlike them it imports the embedded
family's homes only (no `Text.Lex`/`Text.Parse`), so it can itself be
embedded without dragging a source module in behind it.

THE SAFE SUBSET LAW. A general YAML loader's expressive power is TYPE
CONSTRUCTION: a tag names a class and the loader instantiates it
(`!ruby/object`, `!!python/object/apply`), which is arbitrary
behaviour and arbitrary data flowing from an untrusted document into
the process. The safe subset is exactly the value forms the `Json`
family can hold, and the law this module must satisfy is: a document
parses to a finite tree of `Json` nodes, scalars typed only as
integer, float, boolean, null or string, with no node referring to
another, so parsing cannot construct an object, cannot invoke
anything, and cannot produce a cyclic or shared structure. An alias
resolves to a COPY of the anchored node (the safe form: the document's
text expands into data and no reference is materialized), and an alias
that reaches its own anchor is refused rather than expanded forever.
Alias expansion costs what the produced value costs; there is no
separate expansion budget, and the classic self-referential blowup is
the refused cycle, not a memory cliff.

THE ROUND-TRIP LAW. `yaml-print` is total, and `yaml-parse` accepts
its output: `yaml-parse (yaml-print v)` is `Ok v` for every value
whose spine is well formed (`JObj`/`JArr` cons spines ended by
`JObjEnd`/`JArrEnd`), whose floats are finite, and whose strings are
UTF-8. Two renderings carry the round trip on the printer's side:
a string that would re-read as a number, a boolean, null or an
indicator-led token is quoted, so its type survives; a non-finite
float renders `null` (Text.Json's choice, D310), the one rendering
that is not a round trip, and the law covers the finite domain.
Printing is block style (indentation carries the structure the parser
reads), with no trailing newline.

THE KEY LAW. A mapping key is its TEXT: `Json` carries `String` keys
(D148), so a plain key is not typed (`1:` and `"1":` are the same
key, as they are in the JSON data model), and a non-scalar key has no
representation at all and is refused.

NAMED REFUSALS (each is a named error kind with the offending text in
its message, never a silent misread):
  tags of any kind: `!!` and custom `!tag` are refused before the
    node behind them is read, because a tag's only job is to select a
    constructed type (YamlUnsupportedTag);
  object-constructing tags: `!ruby/object` and the runtime-namespace
    families (`ruby/`, `python/`, `perl/`, `java/`, `php/`,
    `dotnet/`, `golang/`, `lua/`) get their own kind, because the
    deserialization CVE this port exists to close is exactly this
    form (YamlObjectTag);
  merge keys `<<`: not implemented, so a plain `<<` key is refused
    rather than kept as an ordinary key (YamlMergeKey);
  multi-document streams: one document per call; a second `---` is
    refused (YamlMultiDocument);
  directives (`%YAML`, `%TAG`): refused, since a directive's whole
    purpose is to change how tags resolve (YamlDirective);
  non-scalar and explicit (`?`) mapping keys (YamlComplexKey);
  duplicate keys in one mapping, refused like Text.Json because
    canonical identity needs unique names (YamlDuplicateKey);
  scalars outside the safe typing: the `.inf`/`.nan` spellings the
    core schema types as non-finite floats, which the finite `JFloat`
    domain cannot carry (YamlUnsupportedScalar);
  an alias with no earlier anchor (YamlUndefinedAlias), and an alias
    that reaches its own anchor (YamlAliasCycle).

BOUNDARIES (forms deliberately narrowed, each a named syntax error
rather than a guess): multi-line plain and quoted scalars (a scalar
closes on its line, which also refuses a folded flow scalar); tabs in
indentation; a block collection starting on its mapping key's line;
a block collection starting on the `---` marker line; a flow mapping
key containing a `:` (a plain flow key ends at its first `:`, so the
1.1-style `{a:1}` reads as `a` -> `1`, while `{http://x: 1}` is a
named syntax error rather than a silent split); sexagesimal and
`0x`/`0o`/`_` separated numeric spellings (the core schema has no
such forms, so they stay strings).

The intermediate parse tree (anchors, aliases, block-scalar lines)
and every constructor of it are module-private (D143's surface rule,
D308's constructor privacy): the surface is `YamlError` plus the two
functions.

</details>

# Types and constructors

<a id="type-59616d6c4572726f72"></a>

## `YamlError`

```cinch
data YamlError
```

[Source](/reference-sources/Text.Yaml.cinch) · line 133

```cinch
YamlError :: (YamlErrorKind -> (Nat -> (Nat -> (String -> YamlError))))
```

<a id="type-59616d6c4572726f724b696e64"></a>

## `YamlErrorKind`

```cinch
data YamlErrorKind
```

The error vocabulary: a named kind, the position, and the text the
refusal is about.

[Source](/reference-sources/Text.Yaml.cinch) · line 119

```cinch
YamlSyntaxError :: YamlErrorKind
```

```cinch
YamlIndentationError :: YamlErrorKind
```

```cinch
YamlUnsupportedTag :: YamlErrorKind
```

```cinch
YamlObjectTag :: YamlErrorKind
```

```cinch
YamlMergeKey :: YamlErrorKind
```

```cinch
YamlMultiDocument :: YamlErrorKind
```

```cinch
YamlUndefinedAlias :: YamlErrorKind
```

```cinch
YamlAliasCycle :: YamlErrorKind
```

```cinch
YamlDirective :: YamlErrorKind
```

```cinch
YamlComplexKey :: YamlErrorKind
```

```cinch
YamlDuplicateKey :: YamlErrorKind
```

```cinch
YamlUnsupportedScalar :: YamlErrorKind
```

# Values

<a id="value-72656e6465722d79616d6c2d6572726f72"></a>

## `render-yaml-error`

```cinch
render-yaml-error :: (YamlError -> String)
```

[Source](/reference-sources/Text.Yaml.cinch) · line 168

<a id="value-79616d6c2d6572726f722d636f6c756d6e"></a>

## `yaml-error-column`

```cinch
yaml-error-column :: (YamlError -> Nat)
```

[Source](/reference-sources/Text.Yaml.cinch) · line 144

<a id="value-79616d6c2d6572726f722d6b696e64"></a>

## `yaml-error-kind`

```cinch
yaml-error-kind :: (YamlError -> YamlErrorKind)
```

[Source](/reference-sources/Text.Yaml.cinch) · line 136

<a id="value-79616d6c2d6572726f722d6c696e65"></a>

## `yaml-error-line`

```cinch
yaml-error-line :: (YamlError -> Nat)
```

[Source](/reference-sources/Text.Yaml.cinch) · line 140

<a id="value-79616d6c2d6572726f722d6d657373616765"></a>

## `yaml-error-message`

```cinch
yaml-error-message :: (YamlError -> String)
```

[Source](/reference-sources/Text.Yaml.cinch) · line 148

<a id="value-79616d6c2d7061727365"></a>

## `yaml-parse`

```cinch
yaml-parse :: (String -> Result YamlError Json)
```

The two surface functions.

[Source](/reference-sources/Text.Yaml.cinch) · line 1726

<a id="value-79616d6c2d7072696e74"></a>

## `yaml-print`

```cinch
yaml-print :: (Json -> String)
```

Printing: block style, indentation carries the structure. Total
(the header's law), so an ill-formed spine prints as one trailing
item or entry under the empty key, like Text.Json's printer.

[Source](/reference-sources/Text.Yaml.cinch) · line 1573

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Text.Yaml.Eq_YamlErrorKind` | `Eq YamlErrorKind` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Yaml.cinch`. Source SHA-256: `2b4ec0c04c4395f8f1b42dd0b2e564ed2453dbc570d827a84fd693f7f1b06589`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
