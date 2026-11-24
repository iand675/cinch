# Effect.FileUtils

Convenience filesystem operations built on the lower-level filesystem API.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.FileUtils</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.FileUtils/source/">View source</a><a href="/packages/search/?q=%2BEffect.FileUtils">Search module</a><a href="/reference-sources/Effect.FileUtils.cinch" download="Effect.FileUtils.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Fs](Effect.Fs.md), [Effect.Random](Effect.Random.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Effect.FileUtils`: [Effect.FileUtils](Effect.FileUtils.md), [Effect.Random](Effect.Random.md).

```sh
cp seed/prelude/Effect/FileUtils.cinch ./Effect.FileUtils.cinch
cp seed/prelude/Effect/Random.cinch ./Effect.Random.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Effect.FileUtils: the mutating filesystem operations (P40, the Ruby
`FileUtils` port named in design/stdlib-parity-ruby.md's OS-utilities
sweep).

Where `Data.Path` carries the pure Pathname operations (basename,
dirname, normalize: text in, text out, no I/O), this home carries the
operations that WRITE to the disk: `mkdir-p`, `rm`/`rm-r`, `cp`,
`mv`, `touch`, and the two temp-name constructors `temp-file` and
`temp-dir`. Every one takes the `Io r` capability and performs
`IO r` (decision D45: the filesystem is consulted at runtime through
the capability, never ambient at compile time; a `stage` body cannot
reach it), and every failure is a NAMED `FileUtilsError` value in a
`Result` (the house style: `Effect.Fs`'s named vocabulary shape),
never a fault.

The operations ride three landed surfaces: `Effect.Fs` (`fs-read`,
`fs-write`, `fs-rename`, `fs-metadata`) for the read, write and
atomic-move path; the `cinch.runtime.fsMkdir`/`fsRemove`/`fsTempDir`
intrinsics (`seed/runtime/src/platform.rs`) for creation and
removal, which `Effect.Fs` does not expose; and `Effect.Random`
(`random-bytes` for the entropy draw and the pure `hex-of-bytes` for
the temp-name nonce). The draw is BOUND to a `let` before rendering,
because the nested `hex-of-bytes (random-bytes n)` shape inside a
capability-constrained function miscompiles to a wrong-word fault
today (a compiler-side bug, reported; the bound shape is verified
good).

Named refusals (boundaries this module deliberately does not cross):
  permissions and ownership: nothing here reads or writes a mode or
    an owner; there is no `chmod`/`chown` (the OS's own permission
    failures surface as `FileUtilsPermissionDenied`, never a change);
  symlink creation: no `symlink` operation exists in the slice (the
    temp-file test constructs its link from the Rust side);
  recursive copy of a symlinked tree: `cp` copies a REGULAR FILE's
    bytes (read plus write) and refuses a directory source
    (`FileUtilsIsADirectory`), so it never walks a tree at all and
    therefore never follows or copies a link;
  timestamp and metadata preservation: `cp` and `touch` do NOT
    preserve or set mtime/atime/mode (`shutil.copy2`'s behavior is
    out of scope; `touch` creates a missing file and leaves an
    existing one's contents unchanged, it does not bump a clock);
  exclusive creation: the slice has no `O_EXCL` create, so
    `temp-file`/`temp-dir` compose a 128-bit random name and then
    CHECK for the name before creating; a name already taken is
    reported as `FileUtilsAlreadyExists`, never silently truncated
    over (the uniqueness argument is the entropy, see `temp-file`).

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

<details class="doc-synopsis" open><summary>Synopsis (9 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-FileUtilsError">FileUtilsError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FileUtilsError</strong></code></li><li><a class="syn-name" href="#v-cp">cp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cp</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-mkdir-p">mkdir-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mkdir-p</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-mv">mv</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mv</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-rm">rm</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rm</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-rm-r">rm-r</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rm-r</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-temp-dir">temp-dir</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">temp-dir</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code></li><li><a class="syn-name" href="#v-temp-file">temp-file</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">temp-file</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code></li><li><a class="syn-name" href="#v-touch">touch</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">touch</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-FileUtilsError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FileUtilsError</strong></code><span class="doc-sig-links"><a href="#t-FileUtilsError" title="Permalink">#</a><a href="/packages/stdlib/Effect.FileUtils/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

What went wrong, named (the shared platform codes `Effect.Fs` maps
its `FsError` from, in this home's vocabulary).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-FileUtilsNotFound"><code><strong class="tok-constructor">FileUtilsNotFound</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a></code></li><li id="k-FileUtilsAlreadyExists"><code><strong class="tok-constructor">FileUtilsAlreadyExists</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a></code></li><li id="k-FileUtilsIsADirectory"><code><strong class="tok-constructor">FileUtilsIsADirectory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a></code></li><li id="k-FileUtilsNotADirectory"><code><strong class="tok-constructor">FileUtilsNotADirectory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a></code></li><li id="k-FileUtilsPermissionDenied"><code><strong class="tok-constructor">FileUtilsPermissionDenied</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a></code></li><li id="k-FileUtilsIoError"><code><strong class="tok-constructor">FileUtilsIoError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-cp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cp</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-cp" title="Permalink">#</a><a href="/packages/stdlib/Effect.FileUtils/source/#L142" title="View source at line 142">Source</a></span></div>
<div class="doc-body">

`cp src dst`: copy a regular file's bytes from `src` to `dst`
(read plus write). A directory `src` is `FileUtilsIsADirectory`,
a missing `src` is `FileUtilsNotFound`, and a `dst` whose parent
is not a directory is `FileUtilsNotADirectory`. The write is
`fs-write`: it creates or truncates `dst` and is NOT atomic (the
atomic replacement is write-then-`fs-rename`, which `mv` uses).

</div>
</article>

<article class="doc-entry" id="v-mkdir-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mkdir-p</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-mkdir-p" title="Permalink">#</a><a href="/packages/stdlib/Effect.FileUtils/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

`mkdir-p p`: create `p` and every missing parent, like `mkdir -p`.
An existing directory is success (idempotent); a non-directory at
`p` is `FileUtilsAlreadyExists`.

</div>
</article>

<article class="doc-entry" id="v-mv">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mv</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-mv" title="Permalink">#</a><a href="/packages/stdlib/Effect.FileUtils/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

`mv src dst`: move `src` to `dst` with `fs-rename`, the POSIX
atomic rename. It is used rather than copy-then-remove because it
is atomic (a concurrent reader sees the whole old or the whole new
state, never a partial) and it preserves the object's identity; the
price is the OS's own rule that a rename cannot cross a filesystem
(that failure surfaces as `FileUtilsIoError`).

</div>
</article>

<article class="doc-entry" id="v-rm">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rm</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-rm" title="Permalink">#</a><a href="/packages/stdlib/Effect.FileUtils/source/#L122" title="View source at line 122">Source</a></span></div>
<div class="doc-body">

`rm p`: remove what `p` names WITHOUT recursion: a regular file,
a symlink (the link itself, never its target: the intrinsic
resolves the kind through `symlink_metadata`), or an empty
directory. A non-empty directory is the OS's refusal
(`FileUtilsIoError`; `rm-r` is the recursive form) and a missing
path is `FileUtilsNotFound`.

</div>
</article>

<article class="doc-entry" id="v-rm-r">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rm-r</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-rm-r" title="Permalink">#</a><a href="/packages/stdlib/Effect.FileUtils/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

`rm-r p`: remove what `p` names recursively: a file, a symlink, or
a whole directory subtree. A symlink is removed as the LINK (the
runtime resolves it through `symlink_metadata` and its recursive
walk does not follow links), so the target is never touched.

</div>
</article>

<article class="doc-entry" id="v-temp-dir">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">temp-dir</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code><span class="doc-sig-links"><a href="#v-temp-dir" title="Permalink">#</a><a href="/packages/stdlib/Effect.FileUtils/source/#L205" title="View source at line 205">Source</a></span></div>
<div class="doc-body">

`temp-dir u`: create (and name) a fresh directory under the
platform temp directory. The name carries the 128-bit nonce and the
directory is created with a bare `mkdir` (no parents), so a name
the OS already knows is reported as `FileUtilsAlreadyExists`
rather than reused.

</div>
</article>

<article class="doc-entry" id="v-temp-file">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">temp-file</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code><span class="doc-sig-links"><a href="#v-temp-file" title="Permalink">#</a><a href="/packages/stdlib/Effect.FileUtils/source/#L227" title="View source at line 227">Source</a></span></div>
<div class="doc-body">

`temp-file u`: create (and name) a fresh EMPTY file under the
platform temp directory. UNIQUENESS: the same 128-bit nonce; the
name is CHECKED with `fs-metadata` before the create, so a name
already taken (a collision the entropy makes astronomically
unlikely, roughly 2^-64 over any realistic number of calls) is
`FileUtilsAlreadyExists`, never a silent truncation of another
file. There is no exclusive-create primitive in the slice, so the
check and the create are not one atomic step; the entropy, not the
check, is what makes a collision negligible, and the refusal is
named rather than guessed away by a retry loop.

</div>
</article>

<article class="doc-entry" id="v-touch">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">touch</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.FileUtils/#t-FileUtilsError" title="Effect.FileUtils.FileUtilsError">FileUtilsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-touch" title="Permalink">#</a><a href="/packages/stdlib/Effect.FileUtils/source/#L170" title="View source at line 170">Source</a></span></div>
<div class="doc-body">

`touch p`: create `p` as an empty file when it does not exist and
leave an existing file's contents unchanged (the cheap touch; it
does NOT bump a timestamp, see the refusals). The create is
`fs-write` of no bytes, so a missing parent is reported.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/FileUtils.cinch`](/packages/stdlib/Effect.FileUtils/source/). Source SHA-256: `26ce540b8755def036a6804f9b2da0db29d81d73bee40ac122c89a5b8ead1d00`.
