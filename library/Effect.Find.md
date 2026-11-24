# Effect.Find

Filesystem walking/search utilities with the module's traversal and error policy.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Find</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Find/">View source</a><a href="/library/search/?q=%2BEffect.Find">Search module</a><a href="/reference-sources/Effect.Find.cinch" download="Effect.Find.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.List](Data.List.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Effect.Fs](Effect.Fs.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Effect.Find`: [Effect.Find](Effect.Find.md).

```sh
cp seed/prelude/Effect/Find.cinch ./Effect.Find.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Effect.Find: the recursive directory walk (P40, the Ruby `Find`
port named in design/stdlib-parity-ruby.md's OS-utilities sweep).

`find` enumerates a subtree depth-first in PRE-ORDER: the root is
reported first, then each directory's entries in `fs-list`'s order
(names ascending, the listing order decision D335 records, recorded
in compiler/CONFORMANCE.md), a directory entered before its next
sibling. That is Ruby's `Find.find` order, and the entry order is
the contract: after a reported directory `d` and before the next
sibling of `d`, every reported path lies beneath `d`.

The visitor is asked once per entry, in that same order, and answers
with a `Step`:
  `Continue` descends into the entry when it is a directory;
  `Prune` keeps the entry and skips its contents (Ruby's
    `Find.prune`; on a non-directory it is a no-op);
  `Stop` ends the walk at that entry, which is the last one
    reported.

Laws:
  `find` reports every visited path exactly once, and reports the
    root first (a root that does not exist is `Err` with nothing
    reported);
  `Prune` at `p` removes exactly the descendants of `p` (when `p`
    names a directory) and moves no other entry;
  `Stop` at `p` makes the result the reported prefix ending at `p`;
  `find-files r` is the `FileIsFile` sublist of the walk that
    descends through everything and `find-dirs r` the
    `FileIsDirectory` one, so a filter changes what is reported,
    never what is visited.

Named refusals (boundaries this module deliberately does not
cross):
  partial results: the first named `FsError` ends the walk and `Err`
    carries nothing collected (a truncated list would be
    indistinguishable from a complete one); a caller who wants the
    frontier reads `fs-list`/`fs-metadata` directly;
  error swallowing: there is no `ignore_error` flag (Ruby's default
    skips a directory it cannot read); a skip would make the result
    a claim about a tree the walk did not read;
  symlink cycles: nothing here detects them, and `fs-metadata`
    reports the target's kind (Effect.Fs's rule), so a directory
    symlink cycle walks until the OS refuses a path; the caller
    breaks it with `Prune`;
  effectful visitors: the visitor is a plain value
    (`Path -> Step`), so the walk's row is the filesystem's `{IO r}`
    alone; a caller whose visit acts collects the paths first and
    acts afterwards;
  a depth bound: the recursion follows the tree's shape, so its
    depth is the tree's.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary, and `Effect.Fs` is the capability walked here.

</details>

<details class="doc-synopsis" open><summary>Synopsis (4 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Step">Step</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Step</strong></code></li><li><a class="syn-name" href="#v-find">find</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">find</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Find/#t-Step" title="Effect.Find.Step">Step</a>) <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a>)</code></li><li><a class="syn-name" href="#v-find-dirs">find-dirs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">find-dirs</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a>)</code></li><li><a class="syn-name" href="#v-find-files">find-files</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">find-files</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Step">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Step</strong></code><span class="doc-sig-links"><a href="#t-Step" title="Permalink">#</a><a href="/library/source/Effect.Find/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

What the visitor decides about the entry it is shown.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Continue"><code><strong class="tok-constructor">Continue</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Find/#t-Step" title="Effect.Find.Step">Step</a></code></li><li id="k-Prune"><code><strong class="tok-constructor">Prune</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Find/#t-Step" title="Effect.Find.Step">Step</a></code></li><li id="k-Stop"><code><strong class="tok-constructor">Stop</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Find/#t-Step" title="Effect.Find.Step">Step</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-find">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">find</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Find/#t-Step" title="Effect.Find.Step">Step</a>) <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a>)</code><span class="doc-sig-links"><a href="#v-find" title="Permalink">#</a><a href="/library/source/Effect.Find/#L148" title="View source at line 148">Source</a></span></div>
<div class="doc-body">

The subtree beneath `root`, pre-order, reporting the paths the
visitor's decisions leave in the walk (see the header for the
order guarantee and the decisions).

</div>
</article>

<article class="doc-entry" id="v-find-dirs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">find-dirs</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a>)</code><span class="doc-sig-links"><a href="#v-find-dirs" title="Permalink">#</a><a href="/library/source/Effect.Find/#L158" title="View source at line 158">Source</a></span></div>
<div class="doc-body">

Every directory beneath `root` (the root included when it is one),
pre-order.

</div>
</article>

<article class="doc-entry" id="v-find-files">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">find-files</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a>)</code><span class="doc-sig-links"><a href="#v-find-files" title="Permalink">#</a><a href="/library/source/Effect.Find/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

Every file beneath `root` (the root itself only when it is one),
pre-order, descending through every directory.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Find.cinch`](/library/source/Effect.Find/). Source SHA-256: `8897fed1fe39856ce9857902d0a5f5fe258a1064bf3be7cf1ba35c033af98080`.
