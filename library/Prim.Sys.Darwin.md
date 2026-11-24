# Prim.Sys.Darwin

The Darwin half of the syscall layer: libSystem calls bound with foreign import ccall (open, read, write, close, lseek, fstat, mkdir, unlink, rename, fsync, getpid), with the -1 result mapped to a named error read from the thread error slot.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Prim.Sys.Darwin</code></div><div class="doc-mod-bar-links"><a href="/library/source/Prim.Sys.Darwin/">View source</a><a href="/library/search/?q=%2BPrim.Sys.Darwin">Search module</a><a href="/reference-sources/Prim.Sys.Darwin.cinch" download="Prim.Sys.Darwin.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Prim.Sys.Darwin: the Darwin filesystem calls, in Cinch.

This module is the Darwin half of the syscall layer the design says
belongs in the language (design/base-as-a-package.md section 5, and the
user's direction that OS access becomes Cinch as the compiler is
bootstrapped). Every entry point here is a `libSystem` symbol bound with
`foreign import ccall`, so this OS needs no assembly at all: the
references it emits resolve to libSystem and the call is the C call.

Verified on macOS arm64: a program that imports this module creates a
file, writes eleven bytes, reads them back through lseek and read, and
gets `SysNotFound` for a path that does not exist.

# The error vocabulary

A `libSystem` call answers `-1` and sets the calling thread's error
slot. Every wrapper turns that into `Err` of `SysError`, whose
constructors `Effect.Fs` maps one for one onto its own named
vocabulary. The numbers are the POSIX six (2, 13, 17, 20, 21, 22) plus
the fallback, so the same mapping serves the Linux module: the two
modules name the same constructors and swap with one import line.

# What is deliberately not here, and why

* Variadic callees. Apple's arm64 ABI passes variadic arguments on the
  stack, and a `foreign import ccall` declaration has no `...`, so the
  third argument of `open` (the creation mode) or of `ioctl` (the
  winsize pointer) would land in a register the callee never reads.
  Creation therefore goes through `creat`, which is a fixed two
  argument call and is exactly `open` with
  `O_WRONLY | O_CREAT | O_TRUNC`, and the terminal control calls stay
  in the Rust runtime until a variadic import form exists.
* Struct layouts. `struct stat`, `struct dirent` and `struct
  sigaction` are data, not entry points. `sys-fstat-into` fills a
  caller's buffer and the caller that owns the layout reads the
  fields, which is the rule `crate::stack` already follows for the
  signal structs.
* Name resolution and process control. They are the next areas; this
  file is the filesystem's, and a module that declares only what one
  area needs keeps the audit local.

</details>

<details class="doc-synopsis"><summary>Synopsis (19 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-SysError">SysError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SysError</strong></code></li><li><a class="syn-name" href="#v-sys-close">sys-close</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-close</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-cpath">sys-cpath</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-cpath</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sys-create">sys-create</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-create</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-errno">sys-errno</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-errno</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code></li><li><a class="syn-name" href="#v-sys-error-of">sys-error-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-error-of</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a></code></li><li><a class="syn-name" href="#v-sys-fstat-into">sys-fstat-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-fstat-into</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-fsync">sys-fsync</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-fsync</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-getpid">sys-getpid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-getpid</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-lseek">sys-lseek</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-lseek</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-mkdir">sys-mkdir</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-mkdir</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-o-rdonly">sys-o-rdonly</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-o-rdonly</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-open-read">sys-open-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-open-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-read">sys-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-rename">sys-rename</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-rename</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-seek-end">sys-seek-end</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-seek-end</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-seek-set">sys-seek-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-seek-set</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-unlink">sys-unlink</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-unlink</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-write">sys-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-SysError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SysError</strong></code><span class="doc-sig-links"><a href="#t-SysError" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

What the operating system said, named. The constructors are the
shared vocabulary `Effect.Fs` presents; its mapping is one for one.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SysNotFound"><code><strong class="tok-constructor">SysNotFound</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a></code></li><li id="k-SysPermissionDenied"><code><strong class="tok-constructor">SysPermissionDenied</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a></code></li><li id="k-SysAlreadyExists"><code><strong class="tok-constructor">SysAlreadyExists</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a></code></li><li id="k-SysNotADirectory"><code><strong class="tok-constructor">SysNotADirectory</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a></code></li><li id="k-SysIsADirectory"><code><strong class="tok-constructor">SysIsADirectory</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a></code></li><li id="k-SysInvalidName"><code><strong class="tok-constructor">SysInvalidName</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a></code></li><li id="k-SysIoError"><code><strong class="tok-constructor">SysIoError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-sys-close">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-close</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-close" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L210" title="View source at line 210">Source</a></span></div>
<div class="doc-body">

Close `fd`.

</div>
</article>

<article class="doc-entry" id="v-sys-cpath">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-cpath</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sys-cpath" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L144" title="View source at line 144">Source</a></span></div>
<div class="doc-body">

`s` as a C string: a fresh buffer one byte longer than the text, the
bytes copied in, the last byte zero. The buffer's address is what a
call takes, and it stays valid while the handle is live.

</div>
</article>

<article class="doc-entry" id="v-sys-create">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-create</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-create" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Create or truncate `path` for writing: the descriptor. This is
`creat`, the fixed argument form of the creating open.

</div>
</article>

<article class="doc-entry" id="v-sys-errno">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-errno</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code><span class="doc-sig-links"><a href="#v-sys-errno" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

The error number the last failed call left, read through the address
`__error` answers. It has to be read immediately after the call that
failed: nothing in this module calls another system function in
between.

</div>
</article>

<article class="doc-entry" id="v-sys-error-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-error-of</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a></code><span class="doc-sig-links"><a href="#v-sys-error-of" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

The error number as the named vocabulary. The six numbers are fixed
by POSIX and identical on Linux; anything else is `SysIoError`.

</div>
</article>

<article class="doc-entry" id="v-sys-fstat-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-fstat-into</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-fstat-into" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L267" title="View source at line 267">Source</a></span></div>
<div class="doc-body">

Fill `buf` with the raw `struct stat` of `fd`. The layout belongs to
the caller that reads the fields out of it.

</div>
</article>

<article class="doc-entry" id="v-sys-fsync">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-fsync</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-fsync" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L258" title="View source at line 258">Source</a></span></div>
<div class="doc-body">

Flush `fd` to the storage device.

</div>
</article>

<article class="doc-entry" id="v-sys-getpid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-getpid</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-getpid" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L275" title="View source at line 275">Source</a></span></div>
<div class="doc-body">

This process's own id.

</div>
</article>

<article class="doc-entry" id="v-sys-lseek">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-lseek</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-lseek" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L219" title="View source at line 219">Source</a></span></div>
<div class="doc-body">

Move `fd`'s offset: the new offset. `whence` is `sys-seek-set` or
`sys-seek-end`.

</div>
</article>

<article class="doc-entry" id="v-sys-mkdir">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-mkdir</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-mkdir" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L239" title="View source at line 239">Source</a></span></div>
<div class="doc-body">

Create the directory `path`. One level only: a missing parent is the
OS's own refusal, and `mkdir -p` is a caller's walk.

</div>
</article>

<article class="doc-entry" id="v-sys-o-rdonly">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-o-rdonly</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-o-rdonly" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

`O_RDONLY`: read only, fixed by the ABI.

</div>
</article>

<article class="doc-entry" id="v-sys-open-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-open-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-open-read" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Open `path` for reading: the descriptor.

</div>
</article>

<article class="doc-entry" id="v-sys-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-read" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L190" title="View source at line 190">Source</a></span></div>
<div class="doc-body">

Read up to `count` bytes of `fd` into `buf` at `at`: the count read,
which is 0 at the end of the stream. The kernel writes the bytes; the
address it writes to is the buffer's own.

</div>
</article>

<article class="doc-entry" id="v-sys-rename">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-rename</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-rename" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L228" title="View source at line 228">Source</a></span></div>
<div class="doc-body">

Rename `from` onto `dst`, atomically (POSIX `rename`: the target sees
the whole old or the whole new content, never a partial).

</div>
</article>

<article class="doc-entry" id="v-sys-seek-end">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-seek-end</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-seek-end" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

`SEEK_END`: an offset from the end.

</div>
</article>

<article class="doc-entry" id="v-sys-seek-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-seek-set</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-seek-set" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

`SEEK_SET`: an offset from the start.

</div>
</article>

<article class="doc-entry" id="v-sys-unlink">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-unlink</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-unlink" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L249" title="View source at line 249">Source</a></span></div>
<div class="doc-body">

Remove the name `path`. This is `unlink`, so a directory answers the
OS's refusal; removing one is the separate directory call.

</div>
</article>

<article class="doc-entry" id="v-sys-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys.Darwin/#t-SysError" title="Prim.Sys.Darwin.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-write" title="Permalink">#</a><a href="/library/source/Prim.Sys.Darwin/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

Write up to `count` bytes of `buf` from `at` to `fd`: the count
written, which may be short (a caller that wants the whole buffer
written loops).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Prim/Sys/Darwin.cinch`](/library/source/Prim.Sys.Darwin/). Source SHA-256: `9bff58d77c388a55aeacf09f90e66094177e2a6ea64025674ee9a20e6c453616`.
