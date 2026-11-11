# Prim.Sys.Linux

The Linux half of the syscall layer: prim bodies that issue syscall with the pinned number per architecture (x86_64 and aarch64), with the kernel negative result mapped to the same named errors as the Darwin module.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Prim.Sys.Linux`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Prim.Sys.Linux.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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
`compiler/tests/prim.rs`'s `platform_gated_asm_bodies_select_per_arch`
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

# Types and constructors

<a id="type-5379734572726f72"></a>

## `SysError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SysError</span></code></pre>

What the kernel said, named. The same constructors as the Darwin
module's, so a caller's mapping is written once.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 79

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysNotFound</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysPermissionDenied</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysAlreadyExists</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysNotADirectory</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysIsADirectory</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysInvalidName</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysIoError</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

# Values

<a id="value-7379732d636c6f7365"></a>

## `sys-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close `fd`.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 353

<a id="value-7379732d6370617468"></a>

## `sys-cpath`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-cpath</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

`s` as a C string, the same shape as the Darwin module's.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 311

<a id="value-7379732d637265617465"></a>

## `sys-create`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-create</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))</code></pre>

Create or truncate `path` for writing: the descriptor.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 334

<a id="value-7379732d6572726f722d6f66"></a>

## `sys-error-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-error-of</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span>)</code></pre>

The kernel's negative return as the named vocabulary. The six numbers
are fixed by POSIX; anything else is `SysIoError`.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 288

<a id="value-7379732d66737461742d696e746f"></a>

## `sys-fstat-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-fstat-into</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>))))</code></pre>

Fill `buf` with the raw `struct stat` of `path`. The layout belongs
to the caller that reads the fields out of it.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 398

<a id="value-7379732d6673796e63"></a>

## `sys-fsync`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-fsync</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Flush `fd` to the storage device.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 391

<a id="value-7379732d676574706964"></a>

## `sys-getpid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-getpid</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">I64</span>)</code></pre>

This process's own id.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 407

<a id="value-7379732d6c7365656b"></a>

## `sys-lseek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-lseek</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))))</code></pre>

Move `fd`'s offset.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 359

<a id="value-7379732d6d6b646972"></a>

## `sys-mkdir`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-mkdir</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Create the directory `path`, one level.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 373

<a id="value-7379732d6f2d72646f6e6c79"></a>

## `sys-o-rdonly`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-o-rdonly</span> :: <span class="tok-constructor">I64</span></code></pre>

`O_RDONLY`.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 94

<a id="value-7379732d6f70656e2d72656164"></a>

## `sys-open-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-open-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))</code></pre>

Open `path` for reading: the descriptor.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 328

<a id="value-7379732d72656164"></a>

## `sys-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>)))))</code></pre>

Read up to `count` bytes of `fd` into `buf` at `at`: the count read.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 340

<a id="value-7379732d72656e616d65"></a>

## `sys-rename`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-rename</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Rename `from` onto `dst`, atomically.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 363

<a id="value-7379732d7365656b2d656e64"></a>

## `sys-seek-end`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-seek-end</span> :: <span class="tok-constructor">I64</span></code></pre>

`SEEK_END`.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 111

<a id="value-7379732d7365656b2d736574"></a>

## `sys-seek-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-seek-set</span> :: <span class="tok-constructor">I64</span></code></pre>

`SEEK_SET`.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 107

<a id="value-7379732d756e6c696e6b"></a>

## `sys-unlink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-unlink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Remove the name `path` with `unlinkat`'s flag word: 0 removes a file
or a link, `sys-at-removedir` (0x200) an empty directory. The flag is
spelled out so the caller owns the choice.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 383

<a id="value-7379732d7772697465"></a>

## `sys-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>)))))</code></pre>

Write up to `count` bytes of `buf` from `at` to `fd`: the count
written.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 347

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Prim/Sys/Linux.cinch`. Source SHA-256: `9cb26e9419f1e5d39ad1f9f7d72b03a30cf463f216e8b26536a802cc3c8839fd`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
