# Text.FilePath

POSIX lexical relative paths, locality tests, filesystem-name conversion and search-list splitting.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.FilePath</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.FilePath/source/">View source</a><a href="/packages/search/?q=%2BText.FilePath">Search module</a><a href="/reference-sources/Text.FilePath.cinch" download="Text.FilePath.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Path](Text.Path.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Text.FilePath`: [Text.FilePath](Text.FilePath.md), [Text.Path](Text.Path.md).

```sh
cp seed/prelude/Text/FilePath.cinch ./Text.FilePath.cinch
cp seed/prelude/Text/Path.cinch ./Text.Path.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Lexical POSIX filesystem paths over UTF-8 text. Relative path calculations
and locality checks use component boundaries and never resolve symbolic links.

</details>

<details class="doc-synopsis" open><summary>Synopsis (4 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-file-path-is-local">file-path-is-local</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">file-path-is-local</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-file-path-localize">file-path-localize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">file-path-localize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-file-path-relative">file-path-relative</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">file-path-relative</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-file-path-split-list">file-path-split-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">file-path-split-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-file-path-is-local">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">file-path-is-local</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-file-path-is-local" title="Permalink">#</a><a href="/packages/stdlib/Text.FilePath/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

A local path is nonempty, relative, and stays beneath its starting directory after lexical cleaning.

</div>
</article>

<article class="doc-entry" id="v-file-path-localize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">file-path-localize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-file-path-localize" title="Permalink">#</a><a href="/packages/stdlib/Text.FilePath/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

Convert a slash-separated filesystem name to POSIX text. The root name is dot.
Every other name requires nonempty elements, no dot or parent elements, and no NUL.

</div>
</article>

<article class="doc-entry" id="v-file-path-relative">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">file-path-relative</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-file-path-relative" title="Permalink">#</a><a href="/packages/stdlib/Text.FilePath/source/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

Compute a lexical route from base to destination. Different roots and unmatched
parent components in the base require knowledge of the working directory.

</div>
</article>

<article class="doc-entry" id="v-file-path-split-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">file-path-split-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-file-path-split-list" title="Permalink">#</a><a href="/packages/stdlib/Text.FilePath/source/#L22" title="View source at line 22">Source</a></span></div>
<div class="doc-body">

An empty POSIX search list has no entries. Colons in nonempty lists preserve empty entries.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/FilePath.cinch`](/packages/stdlib/Text.FilePath/source/). Source SHA-256: `e465f4641efc3618645ed4ff1bfa12c6a1a91c65de4579a2cbc84462642c44a7`.
