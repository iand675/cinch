# Text.Args

Command-line argument processing and its named value/error vocabulary.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Args</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Args/source/">View source</a><a href="/packages/search/?q=%2BText.Args">Search module</a><a href="/reference-sources/Text.Args.cinch" download="Text.Args.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

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
`a` (asserted over sample argvs in seed/tests/json.rs).

Named refusals (unproductions this module deliberately does not
implement; each is a named boundary, never a silent gap):
  short-option clusters (`-abc`) and `-o value` single-dash forms:
    long options only (the Cinch driver spells `--output`);
  option-argument syntax beyond `--name=value` and `--name value`;
  `--name=` with an empty value: the value is the empty string
    (an occurrence, not a refusal).

</details>

<details class="doc-synopsis" open><summary>Synopsis (11 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ArgSpec">ArgSpec</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ArgSpec</strong></code></li><li><a class="syn-name" href="#t-Args">Args</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Args</strong></code></li><li><a class="syn-name" href="#t-ArgsError">ArgsError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ArgsError</strong></code></li><li><a class="syn-name" href="#t-OptVal">OptVal</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OptVal</strong></code></li><li><a class="syn-name" href="#v-args-options">args-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">args-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-Args" title="Text.Args.Args">Args</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-OptVal" title="Text.Args.OptVal">OptVal</a></code></li><li><a class="syn-name" href="#v-args-values">args-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">args-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-Args" title="Text.Args.Args">Args</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-opt-name">opt-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">opt-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-OptVal" title="Text.Args.OptVal">OptVal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-opt-value">opt-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">opt-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-OptVal" title="Text.Args.OptVal">OptVal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-parse-args">parse-args</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-args</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgSpec" title="Text.Args.ArgSpec">ArgSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgsError" title="Text.Args.ArgsError">ArgsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-Args" title="Text.Args.Args">Args</a></code></li><li><a class="syn-name" href="#v-render-args-error">render-args-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-args-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgsError" title="Text.Args.ArgsError">ArgsError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-render-argv">render-argv</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-argv</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-Args" title="Text.Args.Args">Args</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ArgSpec">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ArgSpec</strong></code><span class="doc-sig-links"><a href="#t-ArgSpec" title="Permalink">#</a><a href="/packages/stdlib/Text.Args/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

One option's spec: a flag (no value), a value option, or a
required wrapper around either.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-FlagSpec"><code><strong class="tok-constructor">FlagSpec</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgSpec" title="Text.Args.ArgSpec">ArgSpec</a></code></li><li id="k-ValueSpec"><code><strong class="tok-constructor">ValueSpec</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgSpec" title="Text.Args.ArgSpec">ArgSpec</a></code></li><li id="k-Required"><code><strong class="tok-constructor">Required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgSpec" title="Text.Args.ArgSpec">ArgSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgSpec" title="Text.Args.ArgSpec">ArgSpec</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Args">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Args</strong></code><span class="doc-sig-links"><a href="#t-Args" title="Permalink">#</a><a href="/packages/stdlib/Text.Args/source/#L72" title="View source at line 72">Source</a></span></div>
<div class="doc-body">

The parse result: the option occurrences in argv order and the
positional values in argv order.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Args"><code><strong class="tok-constructor">Args</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-OptVal" title="Text.Args.OptVal">OptVal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-Args" title="Text.Args.Args">Args</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ArgsError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ArgsError</strong></code><span class="doc-sig-links"><a href="#t-ArgsError" title="Permalink">#</a><a href="/packages/stdlib/Text.Args/source/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

The named errors: an option the specs do not know, a value option
without its value, a value on a flag, a required spec that did
not occur.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnknownOption"><code><strong class="tok-constructor">UnknownOption</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgsError" title="Text.Args.ArgsError">ArgsError</a></code></li><li id="k-MissingValue"><code><strong class="tok-constructor">MissingValue</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgsError" title="Text.Args.ArgsError">ArgsError</a></code></li><li id="k-UnexpectedValue"><code><strong class="tok-constructor">UnexpectedValue</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgsError" title="Text.Args.ArgsError">ArgsError</a></code></li><li id="k-MissingRequired"><code><strong class="tok-constructor">MissingRequired</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgsError" title="Text.Args.ArgsError">ArgsError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-OptVal">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OptVal</strong></code><span class="doc-sig-links"><a href="#t-OptVal" title="Permalink">#</a><a href="/packages/stdlib/Text.Args/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

One option occurrence: its name and its value (a flag's value is
`None`).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-OptVal"><code><strong class="tok-constructor">OptVal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-OptVal" title="Text.Args.OptVal">OptVal</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-args-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">args-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-Args" title="Text.Args.Args">Args</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-OptVal" title="Text.Args.OptVal">OptVal</a></code><span class="doc-sig-links"><a href="#v-args-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Args/source/#L75" title="View source at line 75">Source</a></span></div>
</article>

<article class="doc-entry" id="v-args-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">args-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-Args" title="Text.Args.Args">Args</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-args-values" title="Permalink">#</a><a href="/packages/stdlib/Text.Args/source/#L79" title="View source at line 79">Source</a></span></div>
</article>

<article class="doc-entry" id="v-opt-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">opt-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-OptVal" title="Text.Args.OptVal">OptVal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-opt-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Args/source/#L62" title="View source at line 62">Source</a></span></div>
</article>

<article class="doc-entry" id="v-opt-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">opt-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-OptVal" title="Text.Args.OptVal">OptVal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-opt-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Args/source/#L66" title="View source at line 66">Source</a></span></div>
</article>

<article class="doc-entry" id="v-parse-args">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-args</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgSpec" title="Text.Args.ArgSpec">ArgSpec</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgsError" title="Text.Args.ArgsError">ArgsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-Args" title="Text.Args.Args">Args</a></code><span class="doc-sig-links"><a href="#v-parse-args" title="Permalink">#</a><a href="/packages/stdlib/Text.Args/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

The argv walk: options and values accumulate in order; `--` ends
the options.

</div>
</article>

<article class="doc-entry" id="v-render-args-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-args-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-ArgsError" title="Text.Args.ArgsError">ArgsError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-args-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Args/source/#L92" title="View source at line 92">Source</a></span></div>
</article>

<article class="doc-entry" id="v-render-argv">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-argv</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Args/#t-Args" title="Text.Args.Args">Args</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-argv" title="Permalink">#</a><a href="/packages/stdlib/Text.Args/source/#L202" title="View source at line 202">Source</a></span></div>
<div class="doc-body">

The canonical argv of one parse: `--name=value` per value
occurrence, `--name` per flag, then the positional values.

</div>
</article>
