# Text.Toml

The module's supported TOML parsing/rendering surface and explicit unsupported/malformed cases.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Toml</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Toml/">View source</a><a href="/library/search/?q=%2BText.Toml">Search module</a><a href="/reference-sources/Text.Toml.cinch" download="Text.Toml.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Scan](Text.Scan.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Text.Toml`: [Text.Scan](Text.Scan.md), [Text.Toml](Text.Toml.md).

```sh
cp seed/prelude/Text/Scan.cinch ./Text.Scan.cinch
cp seed/prelude/Text/Toml.cinch ./Text.Toml.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-TomlError">TomlError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TomlError</strong></code></li><li><a class="syn-name" href="#v-toml-parse">toml-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">toml-parse</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Toml/#t-TomlError" title="Text.Toml.TomlError">TomlError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-toml-print">toml-print</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">toml-print</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Toml/#t-TomlError" title="Text.Toml.TomlError">TomlError</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-TomlError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TomlError</strong></code><span class="doc-sig-links"><a href="#t-TomlError" title="Permalink">#</a><a href="/library/source/Text.Toml/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

A lexical or grammatical failure: the source line it stands at plus
a human message.
A key (or a table header) that names something already defined.
An out-of-scope date-time or time value (the line it started on).
A value TOML cannot spell: `JNull`, or a non-table document root,
with the reason.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TomlSyntax"><code><strong class="tok-constructor">TomlSyntax</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Toml/#t-TomlError" title="Text.Toml.TomlError">TomlError</a></code></li><li id="k-TomlDuplicateKey"><code><strong class="tok-constructor">TomlDuplicateKey</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Toml/#t-TomlError" title="Text.Toml.TomlError">TomlError</a></code></li><li id="k-TomlDateTime"><code><strong class="tok-constructor">TomlDateTime</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Toml/#t-TomlError" title="Text.Toml.TomlError">TomlError</a></code></li><li id="k-TomlNotRepresentable"><code><strong class="tok-constructor">TomlNotRepresentable</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Toml/#t-TomlError" title="Text.Toml.TomlError">TomlError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-toml-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">toml-parse</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Toml/#t-TomlError" title="Text.Toml.TomlError">TomlError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-toml-parse" title="Permalink">#</a><a href="/library/source/Text.Toml/#L1128" title="View source at line 1128">Source</a></span></div>
</article>

<article class="doc-entry" id="v-toml-print">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">toml-print</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Toml/#t-TomlError" title="Text.Toml.TomlError">TomlError</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-toml-print" title="Permalink">#</a><a href="/library/source/Text.Toml/#L1140" title="View source at line 1140">Source</a></span></div>
<div class="doc-body">

Printing (total, deterministic; emission order stated in the
header).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Toml.cinch`](/library/source/Text.Toml/). Source SHA-256: `a5cd93530d3844f3c904ae55cb8b59a8c4611ba114d319daa937eabad137f7ce`.
