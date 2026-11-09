# Prim.Sys.Darwin

The Darwin half of the syscall layer: libSystem calls bound with foreign import ccall (open, read, write, close, lseek, fstat, mkdir, unlink, rename, fsync, getpid), with the -1 result mapped to a named error read from the thread error slot.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Prim.Sys.Darwin`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Prim.Sys.Darwin.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-5379734572726f72"></a>

## `SysError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SysError</span></code></pre>

What the operating system said, named. The constructors are the
shared vocabulary `Effect.Fs` presents; its mapping is one for one.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 60

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysNotFound</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysPermissionDenied</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysAlreadyExists</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysNotADirectory</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysIsADirectory</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysInvalidName</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysIoError</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span></code></pre>

# Values

<a id="value-7379732d636c6f7365"></a>

## `sys-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close `fd`.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 194

<a id="value-7379732d6370617468"></a>

## `sys-cpath`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-cpath</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

`s` as a C string: a fresh buffer one byte longer than the text, the
bytes copied in, the last byte zero. The buffer's address is what a
call takes, and it stays valid while the handle is live.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 144

<a id="value-7379732d637265617465"></a>

## `sys-create`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-create</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))</code></pre>

Create or truncate `path` for writing: the descriptor. This is
`creat`, the fixed argument form of the creating open.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 163

<a id="value-7379732d6572726e6f"></a>

## `sys-errno`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-errno</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">I32</span>)</code></pre>

The error number the last failed call left, read through the address
`__error` answers. It has to be read immediately after the call that
failed: nothing in this module calls another system function in
between.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 121

<a id="value-7379732d6572726f722d6f66"></a>

## `sys-error-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-error-of</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span>)</code></pre>

The error number as the named vocabulary. The six numbers are fixed
by POSIX and identical on Linux; anything else is `SysIoError`.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 126

<a id="value-7379732d66737461742d696e746f"></a>

## `sys-fstat-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-fstat-into</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Fill `buf` with the raw `struct stat` of `fd`. The layout belongs to
the caller that reads the fields out of it.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 251

<a id="value-7379732d6673796e63"></a>

## `sys-fsync`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-fsync</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Flush `fd` to the storage device.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 242

<a id="value-7379732d676574706964"></a>

## `sys-getpid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-getpid</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">I64</span>)</code></pre>

This process's own id.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 259

<a id="value-7379732d6c7365656b"></a>

## `sys-lseek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-lseek</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))))</code></pre>

Move `fd`'s offset: the new offset. `whence` is `sys-seek-set` or
`sys-seek-end`.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 203

<a id="value-7379732d6d6b646972"></a>

## `sys-mkdir`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-mkdir</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Create the directory `path`. One level only: a missing parent is the
OS's own refusal, and `mkdir -p` is a caller's walk.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 223

<a id="value-7379732d6f2d72646f6e6c79"></a>

## `sys-o-rdonly`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-o-rdonly</span> :: <span class="tok-constructor">I64</span></code></pre>

`O_RDONLY`: read only, fixed by the ABI.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 101

<a id="value-7379732d6f70656e2d72656164"></a>

## `sys-open-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-open-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))</code></pre>

Open `path` for reading: the descriptor.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 153

<a id="value-7379732d72656164"></a>

## `sys-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>)))))</code></pre>

Read up to `count` bytes of `fd` into `buf` at `at`: the count read,
which is 0 at the end of the stream. The kernel writes the bytes; the
address it writes to is the buffer's own.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 174

<a id="value-7379732d72656e616d65"></a>

## `sys-rename`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-rename</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Rename `from` onto `dst`, atomically (POSIX `rename`: the target sees
the whole old or the whole new content, never a partial).

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 212

<a id="value-7379732d7365656b2d656e64"></a>

## `sys-seek-end`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-seek-end</span> :: <span class="tok-constructor">I64</span></code></pre>

`SEEK_END`: an offset from the end.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 109

<a id="value-7379732d7365656b2d736574"></a>

## `sys-seek-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-seek-set</span> :: <span class="tok-constructor">I64</span></code></pre>

`SEEK_SET`: an offset from the start.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 105

<a id="value-7379732d756e6c696e6b"></a>

## `sys-unlink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-unlink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Remove the name `path`. This is `unlink`, so a directory answers the
OS's refusal; removing one is the separate directory call.

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 233

<a id="value-7379732d7772697465"></a>

## `sys-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Darwin</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>)))))</code></pre>

Write up to `count` bytes of `buf` from `at` to `fd`: the count
written, which may be short (a caller that wants the whole buffer
written loops).

[Source](/reference-sources/Prim.Sys.Darwin.cinch) · line 185

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Prim/Sys/Darwin.cinch`. Source SHA-256: `e1b39c28b2ba8667e3f38bbcac810db7321d268b0ee02f55d18173025ced9428`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
