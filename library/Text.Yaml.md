# Text.Yaml

The module's supported YAML subset, structured values, and named parse errors; not an assertion of every YAML feature.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Yaml</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Yaml/">View source</a><a href="/library/search/?q=%2BText.Yaml">Search module</a><a href="/reference-sources/Text.Yaml.cinch" download="Text.Yaml.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Text.Yaml`: [Text.Yaml](Text.Yaml.md).

```sh
cp seed/prelude/Text/Yaml.cinch ./Text.Yaml.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis" open><summary>Synopsis (9 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-YamlError">YamlError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">YamlError</strong></code></li><li><a class="syn-name" href="#t-YamlErrorKind">YamlErrorKind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">YamlErrorKind</strong></code></li><li><a class="syn-name" href="#v-render-yaml-error">render-yaml-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-yaml-error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-yaml-error-column">yaml-error-column</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">yaml-error-column</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-yaml-error-kind">yaml-error-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">yaml-error-kind</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li><a class="syn-name" href="#v-yaml-error-line">yaml-error-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">yaml-error-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-yaml-error-message">yaml-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">yaml-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-yaml-parse">yaml-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">yaml-parse</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-yaml-print">yaml-print</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">yaml-print</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-YamlError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">YamlError</strong></code><span class="doc-sig-links"><a href="#t-YamlError" title="Permalink">#</a><a href="/library/source/Text.Yaml/#L134" title="View source at line 134">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-YamlError"><code><strong class="tok-constructor">YamlError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-YamlErrorKind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">YamlErrorKind</strong></code><span class="doc-sig-links"><a href="#t-YamlErrorKind" title="Permalink">#</a><a href="/library/source/Text.Yaml/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

The error vocabulary: a named kind, the position, and the text the
refusal is about.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-YamlSyntaxError"><code><strong class="tok-constructor">YamlSyntaxError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li id="k-YamlIndentationError"><code><strong class="tok-constructor">YamlIndentationError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li id="k-YamlUnsupportedTag"><code><strong class="tok-constructor">YamlUnsupportedTag</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li id="k-YamlObjectTag"><code><strong class="tok-constructor">YamlObjectTag</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li id="k-YamlMergeKey"><code><strong class="tok-constructor">YamlMergeKey</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li id="k-YamlMultiDocument"><code><strong class="tok-constructor">YamlMultiDocument</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li id="k-YamlUndefinedAlias"><code><strong class="tok-constructor">YamlUndefinedAlias</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li id="k-YamlAliasCycle"><code><strong class="tok-constructor">YamlAliasCycle</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li id="k-YamlDirective"><code><strong class="tok-constructor">YamlDirective</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li id="k-YamlComplexKey"><code><strong class="tok-constructor">YamlComplexKey</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li id="k-YamlDuplicateKey"><code><strong class="tok-constructor">YamlDuplicateKey</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li><li id="k-YamlUnsupportedScalar"><code><strong class="tok-constructor">YamlUnsupportedScalar</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Text.Yaml/#instances">Text.Yaml</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-render-yaml-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-yaml-error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-yaml-error" title="Permalink">#</a><a href="/library/source/Text.Yaml/#L168" title="View source at line 168">Source</a></span></div>
</article>

<article class="doc-entry" id="v-yaml-error-column">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">yaml-error-column</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-yaml-error-column" title="Permalink">#</a><a href="/library/source/Text.Yaml/#L144" title="View source at line 144">Source</a></span></div>
</article>

<article class="doc-entry" id="v-yaml-error-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">yaml-error-kind</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code><span class="doc-sig-links"><a href="#v-yaml-error-kind" title="Permalink">#</a><a href="/library/source/Text.Yaml/#L136" title="View source at line 136">Source</a></span></div>
</article>

<article class="doc-entry" id="v-yaml-error-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">yaml-error-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-yaml-error-line" title="Permalink">#</a><a href="/library/source/Text.Yaml/#L140" title="View source at line 140">Source</a></span></div>
</article>

<article class="doc-entry" id="v-yaml-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">yaml-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-yaml-error-message" title="Permalink">#</a><a href="/library/source/Text.Yaml/#L148" title="View source at line 148">Source</a></span></div>
</article>

<article class="doc-entry" id="v-yaml-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">yaml-parse</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlError" title="Text.Yaml.YamlError">YamlError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-yaml-parse" title="Permalink">#</a><a href="/library/source/Text.Yaml/#L1726" title="View source at line 1726">Source</a></span></div>
<div class="doc-body">

The two surface functions.

</div>
</article>

<article class="doc-entry" id="v-yaml-print">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">yaml-print</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-yaml-print" title="Permalink">#</a><a href="/library/source/Text.Yaml/#L1573" title="View source at line 1573">Source</a></span></div>
<div class="doc-body">

Printing: block style, indentation carries the structure. Total
(the header's law), so an ill-formed spine prints as one trailing
item or entry under the empty key, like Text.Json's printer.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_YamlErrorKind"><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code> <span class="doc-inst-meta">Implicit · <code>Text.Yaml.Eq_YamlErrorKind</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Yaml.cinch`](/library/source/Text.Yaml/). Source SHA-256: `1254ed227cac5d100c4132baa92d4ce0773cc78fd1669b269d022f2d5618e7d8`.
