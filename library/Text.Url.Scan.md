# Text.Url.Scan

Private portable SIMD byte scanning shared by URL parsing, queries and path traversal.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Url.Scan</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Url.Scan/">View source</a><a href="/library/search/?q=%2BText.Url.Scan">Search module</a><a href="/reference-sources/Text.Url.Scan.cinch" download="Text.Url.Scan.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Text.Url.Scan`: [Text.Url.Scan](Text.Url.Scan.md).

```sh
cp seed/prelude/Text/Url/Scan.cinch ./Text.Url.Scan.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Checked byte intervals and retained, exact-load SIMD scans used by URL grammars.
Every returned interval is a fresh buffer. Searches return the interval end
when absent; malformed bounds are rejected by returning the complete input end.

</details>

<details class="doc-synopsis" open><summary>Synopsis (9 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-url-concat">url-concat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-concat</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-copy">url-copy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-copy</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-count-byte">url-count-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-count-byte</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-url-find-byte">url-find-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-find-byte</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-url-find-control">url-find-control</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-find-control</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-url-find-invalid-user">url-find-invalid-user</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-find-invalid-user</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-url-last-byte">url-last-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-last-byte</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-url-scheme-colon">url-scheme-colon</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-scheme-colon</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-url-slice">url-slice</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-slice</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-url-concat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-concat</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-concat" title="Permalink">#</a><a href="/library/source/Text.Url.Scan/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Concatenate chunks with one exact allocation and one checked copy per chunk.

</div>
</article>

<article class="doc-entry" id="v-url-copy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-copy</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-copy" title="Permalink">#</a><a href="/library/source/Text.Url.Scan/#L14" title="View source at line 14">Source</a></span></div>
<div class="doc-body">

Copy a complete input without retaining a mutable caller alias.

</div>
</article>

<article class="doc-entry" id="v-url-count-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-count-byte</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-url-count-byte" title="Permalink">#</a><a href="/library/source/Text.Url.Scan/#L264" title="View source at line 264">Source</a></span></div>
<div class="doc-body">

Count exact wanted-byte occurrences in a half-open interval; invalid bounds yield zero.

</div>
</article>

<article class="doc-entry" id="v-url-find-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-find-byte</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-url-find-byte" title="Permalink">#</a><a href="/library/source/Text.Url.Scan/#L247" title="View source at line 247">Source</a></span></div>
<div class="doc-body">

Find the first wanted byte in a half-open interval, or its end when absent.

</div>
</article>

<article class="doc-entry" id="v-url-find-control">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-find-control</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-url-find-control" title="Permalink">#</a><a href="/library/source/Text.Url.Scan/#L251" title="View source at line 251">Source</a></span></div>
<div class="doc-body">

Find the first ASCII control or DEL in a half-open interval, or its end.

</div>
</article>

<article class="doc-entry" id="v-url-find-invalid-user">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-find-invalid-user</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-url-find-invalid-user" title="Permalink">#</a><a href="/library/source/Text.Url.Scan/#L282" title="View source at line 282">Source</a></span></div>
<div class="doc-body">

Find the first unsupported raw ASCII user-information byte, or the interval end.

</div>
</article>

<article class="doc-entry" id="v-url-last-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-last-byte</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-url-last-byte" title="Permalink">#</a><a href="/library/source/Text.Url.Scan/#L255" title="View source at line 255">Source</a></span></div>
<div class="doc-body">

Find the last wanted byte in a half-open interval, or its end when absent.

</div>
</article>

<article class="doc-entry" id="v-url-scheme-colon">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-scheme-colon</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-url-scheme-colon" title="Permalink">#</a><a href="/library/source/Text.Url.Scan/#L271" title="View source at line 271">Source</a></span></div>
<div class="doc-body">

Find an ASCII scheme colon with SIMD grammar scanning; return input length when absent or invalid.

</div>
</article>

<article class="doc-entry" id="v-url-slice">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-slice</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-slice" title="Permalink">#</a><a href="/library/source/Text.Url.Scan/#L19" title="View source at line 19">Source</a></span></div>
<div class="doc-body">

Copy a half-open interval, returning empty bytes for invalid bounds.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Url/Scan.cinch`](/library/source/Text.Url.Scan/). Source SHA-256: `de30fc49981ed873244a64b34de9a063548776c60264ad69475f040b3118204f`.
