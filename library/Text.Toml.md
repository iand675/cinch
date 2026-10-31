# Text.Toml

The module's supported TOML parsing/rendering surface and explicit unsupported/malformed cases.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Toml`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Toml.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Scan](Text.Scan.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Scan](Text.Scan.md), [Text.Toml](Text.Toml.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Toml: TOML 1.0 text to `Json` values and back, as a subset.

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes. This is the TOML half of the serialization family
curated in design/stdlib-parity-ruby.md (the P37 serialization-breadth
phase, alongside the P31 JSON family). It maps TOML onto the SAME
value family JSON uses (`Data.Json`'s `Json`, decision D148) so the
family stays one vocabulary: TOML strings are `JStr`, integers
`JNum`, floats `JFloat`, booleans `JBool`, arrays the `JArr` spine,
and every table (standard, dotted, inline, and each array-of-tables
element) the `JObj` spine. A document is one `JObj`.

THE LAW SHAPE (parse and print are inverse on emission-order
documents): `toml-parse (toml-print v) == Ok v` for a document already
written in emission order (below). Printing is a total function of the
value alone (every branch is deterministic; the only failure is a
value TOML or the `Json` shape cannot carry, named below), so two
prints of one value are byte identical.

EMISSION ORDER (the determinism, stated): within one table the
entries of the value's `JObj` spine are emitted in three groups, each
preserving the spine order it came from:
  1. scalar and plain-array values, as `key = value` lines;
  2. sub-tables, as a `[dotted.key]` header then that table's own
     entries, recursively;
  3. arrays whose elements are all tables, as one `[[dotted.key]]`
     header block per element.
Header paths are absolute from the document root. Parsing a printed
document rebuilds exactly this spine order (values first, then
sub-tables, then arrays of tables), which is why the round-trip law
above is stated for emission-order documents; a document whose keys
sit in another order prints and re-parses to the same value with its
keys reordered into emission order.

IN SCOPE (the forms this subset parses and prints): bare, quoted and
dotted keys; standard tables (`[a.b]`) and arrays of tables
(`[[a]]`); basic and literal strings with their escapes and both
multi-line forms; decimal integers with underscores and a sign, plus
the hexadecimal (`0x`), octal (`0o`) and binary (`0b`) forms;
floating-point values with a fraction or exponent and `inf`/`nan`;
booleans; arrays (nested, with trailing commas, newlines and
comments); inline tables; and `#` comments.

Named refusals (unproductions this subset deliberately does not
implement; each is a named boundary, never a silent gap):
  offset and local date-times and local times (the four date-time
    lexical forms of TOML 1.0): refused with `TomlDateTime`. Carrying
    them would need the civil-time slice's `Date`/`Time` types (the
    `Json` shape has no date cell) plus TOML's own lexical validation
    (month lengths, leap years, offset ranges) and a decision about
    which cell of the value family holds them;
  `JNull`: TOML 1.0 has no null, so printing refuses it
    (`TomlNotRepresentable`); parsing never produces it;
  a non-table document root: printing refuses every root that is not a
    `JObj`/`JObjEnd` (`TomlNotRepresentable`), since a TOML document
    is a table;
  newlines inside an inline table: TOML 1.0 keeps inline tables on one
    line (the 1.1 relaxation is not adopted), so the parser refuses a
    newline before `}` as a syntax error;
  mixed or heterogeneous arrays are accepted (TOML 1.0 permits them)
    but are printed inline, never as arrays of tables, because only an
    all-table array has a `[[...]]` spelling.

Decision records: this module is the TOML analogue of D148's JSON
family over the same `Json` shape, and it follows D88's runtime
`String` and D106's kebab-case surface.

</details>

# Types and constructors

<a id="type-546f6d6c4572726f72"></a>

## `TomlError`

```cinch
data TomlError
```

The named errors.

A lexical or grammatical failure: the source line it stands at plus
a human message.
A key (or a table header) that names something already defined.
An out-of-scope date-time or time value (the line it started on).
A value TOML cannot spell: `JNull`, or a non-table document root,
with the reason.

[Source](/reference-sources/Text.Toml.cinch) · line 100

```cinch
TomlSyntax :: (Nat -> (String -> TomlError))
```

```cinch
TomlDuplicateKey :: (String -> TomlError)
```

```cinch
TomlDateTime :: (Nat -> TomlError)
```

```cinch
TomlNotRepresentable :: (String -> TomlError)
```

# Values

<a id="value-746f6d6c2d7061727365"></a>

## `toml-parse`

```cinch
toml-parse :: (String -> Result TomlError Json)
```

[Source](/reference-sources/Text.Toml.cinch) · line 1128

<a id="value-746f6d6c2d7072696e74"></a>

## `toml-print`

```cinch
toml-print :: (Json -> Result TomlError String)
```

Printing (total, deterministic; emission order stated in the
header).

[Source](/reference-sources/Text.Toml.cinch) · line 1140

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Toml.cinch`. Source SHA-256: `6b8e9b23caad2de0fe0d06a6f8f975dcdfe435216a22e52a77fbc0aa77899b2e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
