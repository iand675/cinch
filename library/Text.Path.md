# Text.Path

Portable slash-path lexical operations and checked shell patterns with reusable compiled matchers and exact SIMD scans.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Path</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Path/source/">View source</a><a href="/packages/search/?q=%2BText.Path">Search module</a><a href="/reference-sources/Text.Path.cinch" download="Text.Path.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Portable lexical operations and shell patterns for slash-separated UTF-8 paths.
Only / separates components; backslashes, drive letters and NUL are ordinary
path text. Cleaning resolves dots lexically without touching the filesystem.
Matching supports *, ?, escaped literals and nonempty Unicode range classes.
Stars never consume /; classes can explicitly match it. Patterns are checked
completely before matching, including patterns whose first term fails.
Complete scans use inline LLVM SIMD lanes with retained owners and exact tails.

</details>

<details class="doc-synopsis" open><summary>Synopsis (13 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-PathPattern">PathPattern</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PathPattern</strong></code></li><li><a class="syn-name" href="#t-PathPatternError">PathPatternError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PathPatternError</strong></code></li><li><a class="syn-name" href="#v-path-base">path-base</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-path-clean">path-clean</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-clean</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-path-dir">path-dir</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-path-ext">path-ext</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-ext</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-path-is-absolute">path-is-absolute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-is-absolute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-path-join">path-join</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-path-match">path-match</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-match</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Path/#t-PathPatternError" title="Text.Path.PathPatternError">PathPatternError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-path-pattern">path-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Path/#t-PathPatternError" title="Text.Path.PathPatternError">PathPatternError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Path/#t-PathPattern" title="Text.Path.PathPattern">PathPattern</a></code></li><li><a class="syn-name" href="#v-path-pattern-error-message">path-pattern-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-pattern-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Path/#t-PathPatternError" title="Text.Path.PathPatternError">PathPatternError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-path-pattern-match">path-pattern-match</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-pattern-match</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Path/#t-PathPattern" title="Text.Path.PathPattern">PathPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-path-split">path-split</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-split</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-PathPattern">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PathPattern</strong></code><span class="doc-sig-links"><a href="#t-PathPattern" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L354" title="View source at line 354">Source</a></span></div>
<div class="doc-body">

An immutable, fully checked shell pattern reusable across independent path names.

</div>
</article>

<article class="doc-entry" id="t-PathPatternError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PathPatternError</strong></code><span class="doc-sig-links"><a href="#t-PathPatternError" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L334" title="View source at line 334">Source</a></span></div>
<div class="doc-body">

A shell pattern contains an incomplete escape or malformed Unicode range class.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidPathPattern"><code><strong class="tok-constructor">InvalidPathPattern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Path/#t-PathPatternError" title="Text.Path.PathPatternError">PathPatternError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-path-base">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-path-base" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L226" title="View source at line 226">Source</a></span></div>
<div class="doc-body">

Return the final nonempty component, / for slash-only paths, and . for empty input.

</div>
</article>

<article class="doc-entry" id="v-path-clean">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-clean</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-path-clean" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L255" title="View source at line 255">Source</a></span></div>
<div class="doc-body">

Lexically collapse separators and dot components; an empty result becomes .
Rooted parent components stop at /; leading relative parent components are retained.

</div>
</article>

<article class="doc-entry" id="v-path-dir">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-path-dir" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L239" title="View source at line 239">Source</a></span></div>
<div class="doc-body">

Return the cleaned directory portion, using . when no directory is present.

</div>
</article>

<article class="doc-entry" id="v-path-ext">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-ext</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-path-ext" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L250" title="View source at line 250">Source</a></span></div>
<div class="doc-body">

Return the final dot and following bytes of the last component, or empty text.
Leading dots count as extensions; a path ending in / has no extension.

</div>
</article>

<article class="doc-entry" id="v-path-is-absolute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-is-absolute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-path-is-absolute" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L222" title="View source at line 222">Source</a></span></div>
<div class="doc-body">

Whether a path starts with /; no normalization or filesystem lookup is performed.

</div>
</article>

<article class="doc-entry" id="v-path-join">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-path-join" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L327" title="View source at line 327">Source</a></span></div>
<div class="doc-body">

Join nonempty path elements with / and clean the result; all-empty input returns empty text.
Later absolute-looking elements do not discard preceding elements.

</div>
</article>

<article class="doc-entry" id="v-path-match">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-match</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Path/#t-PathPatternError" title="Text.Path.PathPatternError">PathPatternError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-path-match" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L514" title="View source at line 514">Source</a></span></div>
<div class="doc-body">

Check and match a shell pattern against the entire UTF-8 path name.

</div>
</article>

<article class="doc-entry" id="v-path-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Path/#t-PathPatternError" title="Text.Path.PathPatternError">PathPatternError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Path/#t-PathPattern" title="Text.Path.PathPattern">PathPattern</a></code><span class="doc-sig-links"><a href="#v-path-pattern" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L366" title="View source at line 366">Source</a></span></div>
<div class="doc-body">

Parse a complete pattern, rejecting malformed suffixes even when an earlier term cannot match.

</div>
</article>

<article class="doc-entry" id="v-path-pattern-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-pattern-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Path/#t-PathPatternError" title="Text.Path.PathPatternError">PathPatternError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-path-pattern-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L339" title="View source at line 339">Source</a></span></div>
<div class="doc-body">

Render the stable malformed-pattern diagnostic.

</div>
</article>

<article class="doc-entry" id="v-path-pattern-match">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-pattern-match</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Path/#t-PathPattern" title="Text.Path.PathPattern">PathPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-path-pattern-match" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L510" title="View source at line 510">Source</a></span></div>
<div class="doc-body">

Match a reusable checked pattern against an entire path name.
Pattern state and path text remain immutable across repeated calls.

</div>
</article>

<article class="doc-entry" id="v-path-split">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-split</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-path-split" title="Permalink">#</a><a href="/packages/stdlib/Text.Path/source/#L233" title="View source at line 233">Source</a></span></div>
<div class="doc-body">

Split immediately after the final slash, preserving repeated and trailing separators.

</div>
</article>
