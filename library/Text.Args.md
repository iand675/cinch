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

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ArgSpec</span></code></pre>

One option's spec: a flag (no value), a value option, or a
required wrapper around either.

[Source](/reference-sources/Text.Args.cinch) · line 52

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlagSpec</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">ArgSpec</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ValueSpec</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">ArgSpec</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Required</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">ArgSpec</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">ArgSpec</span>)</code></pre>

<a id="type-41726773"></a>

## `Args`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Args</span></code></pre>

The parse result: the option occurrences in argv order and the
positional values in argv order.

[Source](/reference-sources/Text.Args.cinch) · line 72

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Args</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">OptVal</span> -&gt; (<span class="tok-constructor">List</span> <span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">Args</span>))</code></pre>

<a id="type-417267734572726f72"></a>

## `ArgsError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ArgsError</span></code></pre>

The named errors: an option the specs do not know, a value option
without its value, a value on a flag, a required spec that did
not occur.

[Source](/reference-sources/Text.Args.cinch) · line 86

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnknownOption</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">ArgsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MissingValue</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">ArgsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnexpectedValue</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">ArgsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MissingRequired</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">ArgsError</span>)</code></pre>

<a id="type-4f707456616c"></a>

## `OptVal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">OptVal</span></code></pre>

One option occurrence: its name and its value (a flag's value is
`None`).

[Source](/reference-sources/Text.Args.cinch) · line 59

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">OptVal</span> :: (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Option</span> <span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">OptVal</span>))</code></pre>

# Values

<a id="value-617267732d6f7074696f6e73"></a>

## `args-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">args-options</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">Args</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">OptVal</span>))</code></pre>

[Source](/reference-sources/Text.Args.cinch)

<a id="value-617267732d76616c756573"></a>

## `args-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">args-values</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">Args</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Args.cinch)

<a id="value-6f70742d6e616d65"></a>

## `opt-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">opt-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">OptVal</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Args.cinch)

<a id="value-6f70742d76616c7565"></a>

## `opt-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">opt-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">OptVal</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Args.cinch)

<a id="value-70617273652d61726773"></a>

## `parse-args`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-args</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">ArgSpec</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">ArgsError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">Args</span>)))</code></pre>

The argv walk: options and values accumulate in order; `--` ends
the options.

[Source](/reference-sources/Text.Args.cinch) · line 124

<a id="value-72656e6465722d617267732d6572726f72"></a>

## `render-args-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">render-args-error</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">ArgsError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Args.cinch)

<a id="value-72656e6465722d61726776"></a>

## `render-argv`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">render-argv</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Args</span>.<span class="tok-constructor">Args</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

The canonical argv of one parse: `--name=value` per value
occurrence, `--name` per flag, then the positional values.

[Source](/reference-sources/Text.Args.cinch) · line 202

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Args.cinch`. Source SHA-256: `d32463cc68034db08180c0417761348dd772852b1d99a3d24db7d6e3492a91ec`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
