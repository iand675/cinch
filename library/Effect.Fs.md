# Effect.Fs

Filesystem reads/writes, directory and metadata operations, with Path values and named Result errors.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Fs</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Fs/source/">View source</a><a href="/packages/search/?q=%2BEffect.Fs">Search module</a><a href="/reference-sources/Effect.Fs.cinch" download="Effect.Fs.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Prim.Sys](Prim.Sys.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Fs: the filesystem capability (P34, the platform layer).

The operations read, write, list, stat and atomically rename
filesystem bytes, under the `Io r` capability in the mold of the
runtime's `Io r` operations (decision D45: the filesystem is
consulted at runtime through the capability, never ambient at
compile time; a `stage` body cannot reach it, see Effect.Clock's
header). Paths are `Data.Path` values; content is `Bytes`; every
failure is a NAMED `FsError` value in a `Result` (the house style:
`Text.Parse`'s named vocabulary shape), never a fault.

Read and write issue their calls in Cinch through `Prim.Sys`, the
facade over the platform layer: on a Darwin target it re-exports
`Prim.Sys.Darwin`, which binds `open`/`creat`/`read`/`write`/`close`/
`lseek` with `foreign import ccall`, and on a Linux target it
re-exports `Prim.Sys.Linux`, which carries the same entry points as
`prim` bodies that issue `syscall`. Both answer the same `SysError`
constructors, which map one for one onto the named constructors below
(`NotFound` ... `IoError`). The import line here is the same for every
target: the selection is the compile target's, made in the facade.

The remaining operations (atomic rename, metadata, the directory
listing) still cross through the runtime's `cinch_fs_*` word buffers
(`seed/runtime/src/platform.rs`, the plain `foreign import ccall`
machinery): machine scalars only, so paths and content cross through
word buffers filled and drained one byte at a time below. The error
codes those words return map onto the same named constructors, so the
two routes present one vocabulary.

Named refusals (boundaries this module deliberately does not
cross):
  non-atomic write: `fs-write` creates or truncates and is NOT
    atomic; atomic replacement is write-then-`fs-rename` (the probe
    in seed/tests/platform.rs exercises exactly that);
  partial reads: `fs-read` reads the whole file or fails (there is
    no streaming reader in the bootstrap slice);
  permissions beyond the OS's own: the error vocabulary reports
    what the OS said and nothing is widened;
  recursive listing or deletion: `fs-list` names one directory's
    entries (sorted ascending for determinism), and nothing removes
    files;
  symlinks: the OS resolves them at the call (`fs-metadata` reports
    the target's kind).

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

<details class="doc-synopsis" open><summary>Synopsis (10 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-FileKind">FileKind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FileKind</strong></code></li><li><a class="syn-name" href="#t-FileMeta">FileMeta</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FileMeta</strong></code></li><li><a class="syn-name" href="#t-FsError">FsError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FsError</strong></code></li><li><a class="syn-name" href="#v-file-meta-kind">file-meta-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">file-meta-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileMeta" title="Effect.Fs.FileMeta">FileMeta</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileKind" title="Effect.Fs.FileKind">FileKind</a></code></li><li><a class="syn-name" href="#v-file-meta-size">file-meta-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">file-meta-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileMeta" title="Effect.Fs.FileMeta">FileMeta</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-fs-list">fs-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fs-list</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-fs-metadata">fs-metadata</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fs-metadata</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileMeta" title="Effect.Fs.FileMeta">FileMeta</a></code></li><li><a class="syn-name" href="#v-fs-read">fs-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fs-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-fs-rename">fs-rename</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fs-rename</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-fs-write">fs-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fs-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-FileKind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FileKind</strong></code><span class="doc-sig-links"><a href="#t-FileKind" title="Permalink">#</a><a href="/packages/stdlib/Effect.Fs/source/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

What a path names.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-FileIsFile"><code><strong class="tok-constructor">FileIsFile</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileKind" title="Effect.Fs.FileKind">FileKind</a></code></li><li id="k-FileIsDirectory"><code><strong class="tok-constructor">FileIsDirectory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileKind" title="Effect.Fs.FileKind">FileKind</a></code></li><li id="k-FileIsOther"><code><strong class="tok-constructor">FileIsOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileKind" title="Effect.Fs.FileKind">FileKind</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-FileMeta">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FileMeta</strong></code><span class="doc-sig-links"><a href="#t-FileMeta" title="Permalink">#</a><a href="/packages/stdlib/Effect.Fs/source/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

A file's metadata: its kind and its size in bytes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-FileMeta"><code><strong class="tok-constructor">FileMeta</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileKind" title="Effect.Fs.FileKind">FileKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileMeta" title="Effect.Fs.FileMeta">FileMeta</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-FsError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FsError</strong></code><span class="doc-sig-links"><a href="#t-FsError" title="Permalink">#</a><a href="/packages/stdlib/Effect.Fs/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

What went wrong, named (the `std::io::ErrorKind` vocabulary the
runtime maps its codes from).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-NotFound"><code><strong class="tok-constructor">NotFound</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a></code></li><li id="k-PermissionDenied"><code><strong class="tok-constructor">PermissionDenied</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a></code></li><li id="k-AlreadyExists"><code><strong class="tok-constructor">AlreadyExists</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a></code></li><li id="k-NotADirectory"><code><strong class="tok-constructor">NotADirectory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a></code></li><li id="k-IsADirectory"><code><strong class="tok-constructor">IsADirectory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a></code></li><li id="k-InvalidName"><code><strong class="tok-constructor">InvalidName</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a></code></li><li id="k-IoError"><code><strong class="tok-constructor">IoError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-file-meta-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">file-meta-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileMeta" title="Effect.Fs.FileMeta">FileMeta</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileKind" title="Effect.Fs.FileKind">FileKind</a></code><span class="doc-sig-links"><a href="#v-file-meta-kind" title="Permalink">#</a><a href="/packages/stdlib/Effect.Fs/source/#L89" title="View source at line 89">Source</a></span></div>
</article>

<article class="doc-entry" id="v-file-meta-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">file-meta-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileMeta" title="Effect.Fs.FileMeta">FileMeta</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-file-meta-size" title="Permalink">#</a><a href="/packages/stdlib/Effect.Fs/source/#L93" title="View source at line 93">Source</a></span></div>
</article>

<article class="doc-entry" id="v-fs-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fs-list</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-fs-list" title="Permalink">#</a><a href="/packages/stdlib/Effect.Fs/source/#L301" title="View source at line 301">Source</a></span></div>
<div class="doc-body">

The names in the directory `p`, sorted ascending (the OS listing
order is not specified; the sorted listing is this home's
observable order).

</div>
</article>

<article class="doc-entry" id="v-fs-metadata">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fs-metadata</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FileMeta" title="Effect.Fs.FileMeta">FileMeta</a></code><span class="doc-sig-links"><a href="#v-fs-metadata" title="Permalink">#</a><a href="/packages/stdlib/Effect.Fs/source/#L275" title="View source at line 275">Source</a></span></div>
<div class="doc-body">

The kind and size of what `p` names.

</div>
</article>

<article class="doc-entry" id="v-fs-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fs-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-fs-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Fs/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

The whole file at `p`, or the named error (missing paths report
`NotFound`).

The calls are `Prim.Sys`'s: open, the size from `lseek`, one buffer
that large, then a read loop to the buffer's end, then close. The
size is the file's length when the read starts, so a file that grows
while it is being read is read to that length; a file that shrank is
read a second time at its new length, so the answer is exactly what
the kernel produced either way.

</div>
</article>

<article class="doc-entry" id="v-fs-rename">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fs-rename</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-fs-rename" title="Permalink">#</a><a href="/packages/stdlib/Effect.Fs/source/#L262" title="View source at line 262">Source</a></span></div>
<div class="doc-body">

Atomically rename `from` to `to` (POSIX `rename`: the target sees
the whole old or the whole new content, never a partial).

</div>
</article>

<article class="doc-entry" id="v-fs-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fs-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-fs-write" title="Permalink">#</a><a href="/packages/stdlib/Effect.Fs/source/#L242" title="View source at line 242">Source</a></span></div>
<div class="doc-body">

Write `content` to `p`, creating or truncating it (NOT atomic;
atomic replacement is `fs-rename`). The calls are `Prim.Sys`'s:
`creat` (the fixed argument form of the creating open), a write loop
(a write may be short), then close.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Fs.cinch`](/packages/stdlib/Effect.Fs/source/). Source SHA-256: `ef52d29afce18d72843cd2704e3a0ebcbeff428436fa047971807c4c56e93a0a`.
