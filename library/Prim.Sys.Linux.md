# Prim.Sys.Linux

The Linux half of the syscall layer: prim bodies that issue syscall with the pinned number per architecture (x86_64 and aarch64), with the kernel negative result mapped to the same named errors as the Darwin module.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Prim.Sys.Linux</code></div><div class="doc-mod-bar-links"><a href="/library/source/Prim.Sys.Linux/">View source</a><a href="/library/search/?q=%2BPrim.Sys.Linux">Search module</a><a href="/reference-sources/Prim.Sys.Linux.cinch" download="Prim.Sys.Linux.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Prim.Sys.Linux: the Linux filesystem calls, in Cinch, on the `syscall`
instruction.

This module is the Linux half of the syscall layer the design says
belongs in the language (design/base-as-a-package.md section 5). Unlike
Darwin, Linux has a stable syscall ABI, so there is no C library in the
path: every entry here is a `prim` whose `unsafe llvm` body issues
`syscall` (x86_64) or `svc #0` (aarch64) with the number pinned in the
body itself.

# NOT EXECUTED ON ANY MACHINE AS OF THIS WRITING

The development machine is macOS arm64, so this module has been
checked, emitted, assembled and linked but never run: its bodies would
issue Linux syscall numbers to the Darwin kernel and trap. Treat the
numbers below as unverified until a Linux machine runs a program that
imports this module. The arch that emitted here is aarch64
(`cinch emit-ir` on a program that imports the module shows
`call i64 asm sideeffect "mov x0, ... svc #0 ..."`).

# One file, two architectures

A prim body can be gated on the compile target: `when arch aarch64`,
`when os linux`, or both as one body's conjunction, and the checker
selects the matching body against the target profile (pinned by
`seed/tests/prim.rs`'s `platform_gated_asm_bodies_select_per_arch`
and `clauses_on_one_body_are_a_conjunction`). That is why the two
architectures live in one module here: the architecture is expressible
in the body, so no per-architecture split is needed. `Prim.Sys.Darwin`
is a separate module because its implementation differs in kind (a
libSystem call rather than a `syscall` instruction), not because a
`when` clause cannot see the operating system.

Every primitive also carries a portable baseline body (the `llvm` one
after the two `when` bodies) that answers the kernel's `ENOSYS`, 38
negated. On an architecture with no pinned numbers that is what a call
reports: "this route is not implemented here", which is a named `Err`
rather than a wrong number sent to a kernel. The baseline is also what
keeps this module checkable, and therefore part of the prelude, on a
target such as wasm32 that has neither body.

# The clobbers are the point

`syscall` destroys `rcx` and `r11` on x86_64 and the number register on
aarch64, so every constraint string names them (`~{rcx}`, `~{r11}`,
`~{x8}`) beside `~{memory}`. The `asm { ... }` body form takes its
constraints from the parameter classes only and cannot name a clobber,
which is why these bodies are written as `unsafe llvm` with the
constraints spelled out. `\0A` is the newline separator inside a `llvm`
body and survives the lexer because that body is verbatim text.

# The error convention

The kernel answers `-errno` in the return register, so the wrapper
compares the raw value (there is no error slot to read on this path)
and maps the six POSIX-fixed numbers onto `SysError`, whose
constructors `Effect.Fs` maps onto its own named vocabulary. The
constructor names are the same as `Prim.Sys.Darwin`'s, so the two
modules swap with one import line.

</details>

<details class="doc-synopsis" open><summary>Synopsis (18 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-SysError">SysError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SysError</strong></code></li><li><a class="syn-name" href="#v-sys-close">sys-close</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-close</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-cpath">sys-cpath</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-cpath</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sys-create">sys-create</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-create</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-error-of">sys-error-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-error-of</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a></code></li><li><a class="syn-name" href="#v-sys-fstat-into">sys-fstat-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-fstat-into</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-fsync">sys-fsync</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-fsync</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-getpid">sys-getpid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-getpid</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-lseek">sys-lseek</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-lseek</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-mkdir">sys-mkdir</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-mkdir</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-o-rdonly">sys-o-rdonly</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-o-rdonly</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-open-read">sys-open-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-open-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-read">sys-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-rename">sys-rename</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-rename</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-seek-end">sys-seek-end</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-seek-end</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-seek-set">sys-seek-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-seek-set</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sys-unlink">sys-unlink</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-unlink</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sys-write">sys-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sys-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-SysError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SysError</strong></code><span class="doc-sig-links"><a href="#t-SysError" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

What the kernel said, named. The same constructors as the Darwin
module's, so a caller's mapping is written once.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SysNotFound"><code><strong class="tok-constructor">SysNotFound</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a></code></li><li id="k-SysPermissionDenied"><code><strong class="tok-constructor">SysPermissionDenied</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a></code></li><li id="k-SysAlreadyExists"><code><strong class="tok-constructor">SysAlreadyExists</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a></code></li><li id="k-SysNotADirectory"><code><strong class="tok-constructor">SysNotADirectory</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a></code></li><li id="k-SysIsADirectory"><code><strong class="tok-constructor">SysIsADirectory</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a></code></li><li id="k-SysInvalidName"><code><strong class="tok-constructor">SysInvalidName</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a></code></li><li id="k-SysIoError"><code><strong class="tok-constructor">SysIoError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-sys-close">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-close</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-close" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L353" title="View source at line 353">Source</a></span></div>
<div class="doc-body">

Close `fd`.

</div>
</article>

<article class="doc-entry" id="v-sys-cpath">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-cpath</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sys-cpath" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L311" title="View source at line 311">Source</a></span></div>
<div class="doc-body">

`s` as a C string, the same shape as the Darwin module's.

</div>
</article>

<article class="doc-entry" id="v-sys-create">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-create</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-create" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L334" title="View source at line 334">Source</a></span></div>
<div class="doc-body">

Create or truncate `path` for writing: the descriptor.

</div>
</article>

<article class="doc-entry" id="v-sys-error-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-error-of</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a></code><span class="doc-sig-links"><a href="#v-sys-error-of" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L288" title="View source at line 288">Source</a></span></div>
<div class="doc-body">

The kernel's negative return as the named vocabulary. The six numbers
are fixed by POSIX; anything else is `SysIoError`.

</div>
</article>

<article class="doc-entry" id="v-sys-fstat-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-fstat-into</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-fstat-into" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L398" title="View source at line 398">Source</a></span></div>
<div class="doc-body">

Fill `buf` with the raw `struct stat` of `path`. The layout belongs
to the caller that reads the fields out of it.

</div>
</article>

<article class="doc-entry" id="v-sys-fsync">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-fsync</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-fsync" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L391" title="View source at line 391">Source</a></span></div>
<div class="doc-body">

Flush `fd` to the storage device.

</div>
</article>

<article class="doc-entry" id="v-sys-getpid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-getpid</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-getpid" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L407" title="View source at line 407">Source</a></span></div>
<div class="doc-body">

This process's own id.

</div>
</article>

<article class="doc-entry" id="v-sys-lseek">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-lseek</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-lseek" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L359" title="View source at line 359">Source</a></span></div>
<div class="doc-body">

Move `fd`'s offset.

</div>
</article>

<article class="doc-entry" id="v-sys-mkdir">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-mkdir</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-mkdir" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L373" title="View source at line 373">Source</a></span></div>
<div class="doc-body">

Create the directory `path`, one level.

</div>
</article>

<article class="doc-entry" id="v-sys-o-rdonly">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-o-rdonly</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-o-rdonly" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L94" title="View source at line 94">Source</a></span></div>
<div class="doc-body">

`O_RDONLY`.

</div>
</article>

<article class="doc-entry" id="v-sys-open-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-open-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-open-read" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L328" title="View source at line 328">Source</a></span></div>
<div class="doc-body">

Open `path` for reading: the descriptor.

</div>
</article>

<article class="doc-entry" id="v-sys-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-read" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L340" title="View source at line 340">Source</a></span></div>
<div class="doc-body">

Read up to `count` bytes of `fd` into `buf` at `at`: the count read.

</div>
</article>

<article class="doc-entry" id="v-sys-rename">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-rename</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-rename" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L363" title="View source at line 363">Source</a></span></div>
<div class="doc-body">

Rename `from` onto `dst`, atomically.

</div>
</article>

<article class="doc-entry" id="v-sys-seek-end">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-seek-end</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-seek-end" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

`SEEK_END`.

</div>
</article>

<article class="doc-entry" id="v-sys-seek-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-seek-set</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-seek-set" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

`SEEK_SET`.

</div>
</article>

<article class="doc-entry" id="v-sys-unlink">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-unlink</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sys-unlink" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L383" title="View source at line 383">Source</a></span></div>
<div class="doc-body">

Remove the name `path` with `unlinkat`'s flag word: 0 removes a file
or a link, `sys-at-removedir` (0x200) an empty directory. The flag is
spelled out so the caller owns the choice.

</div>
</article>

<article class="doc-entry" id="v-sys-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sys-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Prim.Sys/#t-SysError" title="Prim.Sys.Linux.SysError">SysError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sys-write" title="Permalink">#</a><a href="/library/source/Prim.Sys.Linux/#L347" title="View source at line 347">Source</a></span></div>
<div class="doc-body">

Write up to `count` bytes of `buf` from `at` to `fd`: the count
written.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Prim/Sys/Linux.cinch`](/library/source/Prim.Sys.Linux/). Source SHA-256: `cf0a9233c9c21c13faa3f5978d0787dd8cfefe0bc5f47b054997800408670a4d`.
