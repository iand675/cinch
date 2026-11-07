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

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Converter</span></code></pre>

The value converter family. `ConvFlag` takes no value: its presence
binds `VBool True`, which is the whole of a flag's information.

[Source](/reference-sources/Text.OptionParser.cinch) · line 98

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ConvString</span> :: <span class="tok-constructor">Converter</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ConvInt</span> :: <span class="tok-constructor">Converter</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ConvFloat</span> :: <span class="tok-constructor">Converter</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ConvFlag</span> :: <span class="tok-constructor">Converter</span></code></pre>

<a id="type-4f7074696f6e50617273654572726f72"></a>

## `OptionParseError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">OptionParseError</span></code></pre>

One error constructor per refusal: an unknown option (the spelling is
carried verbatim), a value option whose value is missing, a value that
fails its converter (the name and the offending spelling), an
ambiguous long-name prefix (the prefix and its candidates, in spec
order), and a value on a flag.

[Source](/reference-sources/Text.OptionParser.cinch) · line 226

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnknownOption</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">OptionParseError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MissingArgument</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">OptionParseError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidValue</span> :: (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">OptionParseError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AmbiguousOption</span> :: (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">List</span> <span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">OptionParseError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnexpectedValue</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">OptionParseError</span>)</code></pre>

<a id="type-4f7074696f6e53706563"></a>

## `OptionSpec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">OptionSpec</span></code></pre>

One option's spec: an optional single-character short name, the
required long name (the binding key), the converter, and the help line
for `option-help`. `spec-takes-value` is derived from the converter
(every converter but `ConvFlag` consumes a value).

[Source](/reference-sources/Text.OptionParser.cinch) · line 108

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">OptSpec</span> :: (<span class="tok-constructor">Option</span> <span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Converter</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">OptionSpec</span>))))</code></pre>

<a id="type-4f7074696f6e56616c7565"></a>

## `OptionValue`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">OptionValue</span></code></pre>

A bound value is one of the four converter images. `VBool` carries a
flag's presence.

[Source](/reference-sources/Text.OptionParser.cinch) · line 141

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">VString</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">OptionValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">VInt</span> :: (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">OptionValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">VFloat</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">OptionValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">VBool</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">OptionValue</span>)</code></pre>

<a id="type-5061727365644f7074696f6e73"></a>

## `ParsedOptions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ParsedOptions</span></code></pre>

The parse result: the bindings keyed by long name (each name once,
holding its LAST occurrence) and the positional values in argv order.
Flags bind `VBool True`; `option-flags` is the flag subset of that
list.

[Source](/reference-sources/Text.OptionParser.cinch) · line 189

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ParsedOptions</span> :: (<span class="tok-constructor">List</span> (<span class="tok-constructor">String</span>, <span class="tok-constructor">OptionValue</span>) -&gt; (<span class="tok-constructor">List</span> <span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">ParsedOptions</span>))</code></pre>

# Values

<a id="value-6f7074696f6e2d666c61672d70726573656e74"></a>

## `option-flag-present`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">option-flag-present</span> :: (<span class="tok-constructor">ParsedOptions</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-6f7074696f6e2d666c616773"></a>

## `option-flags`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">option-flags</span> :: (<span class="tok-constructor">ParsedOptions</span> -&gt; <span class="tok-constructor">List</span> <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-6f7074696f6e2d686173"></a>

## `option-has`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">option-has</span> :: (<span class="tok-constructor">ParsedOptions</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-6f7074696f6e2d68656c70"></a>

## `option-help`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">option-help</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">OptionSpec</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

`option-help` renders one row per spec: the short/long spellings and
the value placeholder in a padded left column, then the help text, so
the rendered table names every declared option.

[Source](/reference-sources/Text.OptionParser.cinch) · line 529

<a id="value-6f7074696f6e2d706f736974696f6e616c73"></a>

## `option-positionals`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">option-positionals</span> :: (<span class="tok-constructor">ParsedOptions</span> -&gt; <span class="tok-constructor">List</span> <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-6f7074696f6e2d76616c7565"></a>

## `option-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">option-value</span> :: (<span class="tok-constructor">ParsedOptions</span> -&gt; (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">OptionValue</span>))</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-70617273652d6f7074696f6e73"></a>

## `parse-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-options</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">OptionSpec</span> -&gt; (<span class="tok-constructor">List</span> <span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">OptionParseError</span> <span class="tok-constructor">ParsedOptions</span>))</code></pre>

The parse.

[Source](/reference-sources/Text.OptionParser.cinch) · line 246

<a id="value-72656e6465722d6f7074696f6e2d70617273652d6572726f72"></a>

## `render-option-parse-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">render-option-parse-error</span> :: (<span class="tok-constructor">OptionParseError</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-737065632d636f6e766572746572"></a>

## `spec-converter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">spec-converter</span> :: (<span class="tok-constructor">OptionSpec</span> -&gt; <span class="tok-constructor">Converter</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-737065632d68656c70"></a>

## `spec-help`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">spec-help</span> :: (<span class="tok-constructor">OptionSpec</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-737065632d6c6f6e67"></a>

## `spec-long`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">spec-long</span> :: (<span class="tok-constructor">OptionSpec</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-737065632d73686f7274"></a>

## `spec-short`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">spec-short</span> :: (<span class="tok-constructor">OptionSpec</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-737065632d74616b65732d76616c7565"></a>

## `spec-takes-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">spec-takes-value</span> :: (<span class="tok-constructor">OptionSpec</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-76616c75652d626f6f6c"></a>

## `value-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-bool</span> :: (<span class="tok-constructor">OptionValue</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-76616c75652d666c6f6174"></a>

## `value-float`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-float</span> :: (<span class="tok-constructor">OptionValue</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-76616c75652d696e74"></a>

## `value-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-int</span> :: (<span class="tok-constructor">OptionValue</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Int</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-76616c75652d737472696e67"></a>

## `value-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-string</span> :: (<span class="tok-constructor">OptionValue</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.OptionParser.cinch)

<a id="value-76616c75652d746f2d737472696e67"></a>

## `value-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-to-string</span> :: (<span class="tok-constructor">OptionValue</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

The rendering (the `Text.*` conversion direction): a value's text for
diagnostics. The float uses the runtime's shortest round-trip decimal
(the same spelling `Data.Machine.f64-to-decimal` gives, which `Json`'s
printer uses).

[Source](/reference-sources/Text.OptionParser.cinch) · line 171

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/OptionParser.cinch`. Source SHA-256: `f6f1d8610648fed5f3355567cdd52a381f80322f8996ce29482931ef0977cc48`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
