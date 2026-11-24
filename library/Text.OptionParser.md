# Text.OptionParser

Declarative option specifications, short/long forms, conversion, positionals, usage, and named parse errors.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.OptionParser</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.OptionParser/source/">View source</a><a href="/packages/search/?q=%2BText.OptionParser">Search module</a><a href="/reference-sources/Text.OptionParser.cinch" download="Text.OptionParser.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Scan](Text.Scan.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Text.OptionParser`: [Text.OptionParser](Text.OptionParser.md), [Text.Scan](Text.Scan.md).

```sh
cp seed/prelude/Text/OptionParser.cinch ./Text.OptionParser.cinch
cp seed/prelude/Text/Scan.cinch ./Text.Scan.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (23 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Converter">Converter</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Converter</strong></code></li><li><a class="syn-name" href="#t-OptionParseError">OptionParseError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OptionParseError</strong></code></li><li><a class="syn-name" href="#t-OptionSpec">OptionSpec</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OptionSpec</strong></code></li><li><a class="syn-name" href="#t-OptionValue">OptionValue</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OptionValue</strong></code></li><li><a class="syn-name" href="#t-ParsedOptions">ParsedOptions</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ParsedOptions</strong></code></li><li><a class="syn-name" href="#v-option-flag-present">option-flag-present</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">option-flag-present</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-option-flags">option-flags</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">option-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-option-has">option-has</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">option-has</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-option-help">option-help</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">option-help</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-option-positionals">option-positionals</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">option-positionals</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-option-value">option-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">option-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a></code></li><li><a class="syn-name" href="#v-parse-options">parse-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionParseError" title="Text.OptionParser.OptionParseError">OptionParseError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a></code></li><li><a class="syn-name" href="#v-render-option-parse-error">render-option-parse-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-option-parse-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionParseError" title="Text.OptionParser.OptionParseError">OptionParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-spec-converter">spec-converter</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">spec-converter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-Converter" title="Text.OptionParser.Converter">Converter</a></code></li><li><a class="syn-name" href="#v-spec-help">spec-help</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">spec-help</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-spec-long">spec-long</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">spec-long</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-spec-short">spec-short</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">spec-short</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-spec-takes-value">spec-takes-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">spec-takes-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-value-bool">value-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-value-float">value-float</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-float</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-value-int">value-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-value-string">value-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-value-to-string">value-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Converter">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Converter</strong></code><span class="doc-sig-links"><a href="#t-Converter" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

The value converter family. `ConvFlag` takes no value: its presence
binds `VBool True`, which is the whole of a flag's information.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ConvString"><code><strong class="tok-constructor">ConvString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-Converter" title="Text.OptionParser.Converter">Converter</a></code></li><li id="k-ConvInt"><code><strong class="tok-constructor">ConvInt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-Converter" title="Text.OptionParser.Converter">Converter</a></code></li><li id="k-ConvFloat"><code><strong class="tok-constructor">ConvFloat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-Converter" title="Text.OptionParser.Converter">Converter</a></code></li><li id="k-ConvFlag"><code><strong class="tok-constructor">ConvFlag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-Converter" title="Text.OptionParser.Converter">Converter</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-OptionParseError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OptionParseError</strong></code><span class="doc-sig-links"><a href="#t-OptionParseError" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L226" title="View source at line 226">Source</a></span></div>
<div class="doc-body">

One error constructor per refusal: an unknown option (the spelling is
carried verbatim), a value option whose value is missing, a value that
fails its converter (the name and the offending spelling), an
ambiguous long-name prefix (the prefix and its candidates, in spec
order), and a value on a flag.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnknownOption"><code><strong class="tok-constructor">UnknownOption</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionParseError" title="Text.OptionParser.OptionParseError">OptionParseError</a></code></li><li id="k-MissingArgument"><code><strong class="tok-constructor">MissingArgument</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionParseError" title="Text.OptionParser.OptionParseError">OptionParseError</a></code></li><li id="k-InvalidValue"><code><strong class="tok-constructor">InvalidValue</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionParseError" title="Text.OptionParser.OptionParseError">OptionParseError</a></code></li><li id="k-AmbiguousOption"><code><strong class="tok-constructor">AmbiguousOption</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionParseError" title="Text.OptionParser.OptionParseError">OptionParseError</a></code></li><li id="k-UnexpectedValue"><code><strong class="tok-constructor">UnexpectedValue</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionParseError" title="Text.OptionParser.OptionParseError">OptionParseError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-OptionSpec">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OptionSpec</strong></code><span class="doc-sig-links"><a href="#t-OptionSpec" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

One option's spec: an optional single-character short name, the
required long name (the binding key), the converter, and the help line
for `option-help`. `spec-takes-value` is derived from the converter
(every converter but `ConvFlag` consumes a value).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-OptSpec"><code><strong class="tok-constructor">OptSpec</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-Converter" title="Text.OptionParser.Converter">Converter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-OptionValue">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OptionValue</strong></code><span class="doc-sig-links"><a href="#t-OptionValue" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

A bound value is one of the four converter images. `VBool` carries a
flag's presence.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-VString"><code><strong class="tok-constructor">VString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a></code></li><li id="k-VInt"><code><strong class="tok-constructor">VInt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a></code></li><li id="k-VFloat"><code><strong class="tok-constructor">VFloat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a></code></li><li id="k-VBool"><code><strong class="tok-constructor">VBool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ParsedOptions">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ParsedOptions</strong></code><span class="doc-sig-links"><a href="#t-ParsedOptions" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L189" title="View source at line 189">Source</a></span></div>
<div class="doc-body">

The parse result: the bindings keyed by long name (each name once,
holding its LAST occurrence) and the positional values in argv order.
Flags bind `VBool True`; `option-flags` is the flag subset of that
list.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ParsedOptions"><code><strong class="tok-constructor">ParsedOptions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-option-flag-present">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">option-flag-present</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-option-flag-present" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L201" title="View source at line 201">Source</a></span></div>
</article>

<article class="doc-entry" id="v-option-flags">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">option-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-option-flags" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L208" title="View source at line 208">Source</a></span></div>
</article>

<article class="doc-entry" id="v-option-has">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">option-has</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-option-has" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L196" title="View source at line 196">Source</a></span></div>
</article>

<article class="doc-entry" id="v-option-help">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">option-help</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-option-help" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L529" title="View source at line 529">Source</a></span></div>
<div class="doc-body">

`option-help` renders one row per spec: the short/long spellings and
the value placeholder in a padded left column, then the help text, so
the rendered table names every declared option.

</div>
</article>

<article class="doc-entry" id="v-option-positionals">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">option-positionals</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-option-positionals" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L212" title="View source at line 212">Source</a></span></div>
</article>

<article class="doc-entry" id="v-option-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">option-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a></code><span class="doc-sig-links"><a href="#v-option-value" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L192" title="View source at line 192">Source</a></span></div>
</article>

<article class="doc-entry" id="v-parse-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionParseError" title="Text.OptionParser.OptionParseError">OptionParseError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-ParsedOptions" title="Text.OptionParser.ParsedOptions">ParsedOptions</a></code><span class="doc-sig-links"><a href="#v-parse-options" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L246" title="View source at line 246">Source</a></span></div>
<div class="doc-body">

The parse.

</div>
</article>

<article class="doc-entry" id="v-render-option-parse-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-option-parse-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionParseError" title="Text.OptionParser.OptionParseError">OptionParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-option-parse-error" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L233" title="View source at line 233">Source</a></span></div>
</article>

<article class="doc-entry" id="v-spec-converter">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">spec-converter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-Converter" title="Text.OptionParser.Converter">Converter</a></code><span class="doc-sig-links"><a href="#v-spec-converter" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L119" title="View source at line 119">Source</a></span></div>
</article>

<article class="doc-entry" id="v-spec-help">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">spec-help</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-spec-help" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L123" title="View source at line 123">Source</a></span></div>
</article>

<article class="doc-entry" id="v-spec-long">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">spec-long</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-spec-long" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L115" title="View source at line 115">Source</a></span></div>
</article>

<article class="doc-entry" id="v-spec-short">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">spec-short</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-spec-short" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L111" title="View source at line 111">Source</a></span></div>
</article>

<article class="doc-entry" id="v-spec-takes-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">spec-takes-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionSpec" title="Text.OptionParser.OptionSpec">OptionSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-spec-takes-value" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L127" title="View source at line 127">Source</a></span></div>
</article>

<article class="doc-entry" id="v-value-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-value-bool" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L162" title="View source at line 162">Source</a></span></div>
</article>

<article class="doc-entry" id="v-value-float">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-float</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-value-float" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L157" title="View source at line 157">Source</a></span></div>
</article>

<article class="doc-entry" id="v-value-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-value-int" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L152" title="View source at line 152">Source</a></span></div>
</article>

<article class="doc-entry" id="v-value-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-value-string" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L147" title="View source at line 147">Source</a></span></div>
</article>

<article class="doc-entry" id="v-value-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.OptionParser/#t-OptionValue" title="Text.OptionParser.OptionValue">OptionValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-value-to-string" title="Permalink">#</a><a href="/packages/stdlib/Text.OptionParser/source/#L171" title="View source at line 171">Source</a></span></div>
<div class="doc-body">

The rendering (the `Text.*` conversion direction): a value's text for
diagnostics. The float uses the runtime's shortest round-trip decimal
(the same spelling `Data.Machine.f64-to-decimal` gives, which `Json`'s
printer uses).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/OptionParser.cinch`](/packages/stdlib/Text.OptionParser/source/). Source SHA-256: `a98ed14dc7c9027ecc53724e553eaf38be5066a392559916d31036bf5f557eee`.
